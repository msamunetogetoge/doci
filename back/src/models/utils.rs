use crate::models::schemas::*;
use dotenv::dotenv;
// use serde::{de::value::StrDeserializer, Deserialize, Serialize};
use sqlx::{postgres::PgPool};
use std::{env, fs, io, path::{Path, PathBuf}};

/**
ページ階層構造から、app_id を取り出す
もしも、二つ以上のapp_idが紛れていたらpanicする
*/
pub fn get_app_id(pages: &[PageHierarchy]) -> i64 {
    let first_page = pages.first().unwrap();
    let i = first_page.app_id;

    if pages.iter().all(|p| p.app_id == i) {
        i
    } else {
        panic!("app_id が複数あるデータが入力された。")
    }
}

// Markdownを補完するディレクトリを作成する
pub fn models_init() {
    let folder_path = env::current_dir().unwrap().join(Path::new("md"));

    if let Err(why) = fs::create_dir(folder_path) {
        if why.kind() != io::ErrorKind::AlreadyExists {
            println!("! {:?}", why.kind());
            panic!("Folder is not Exist But Cannot create folder")
        }
    }
}

// .envファイルの情報からdbに接続する
pub async fn get_conn() -> PgPool {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgPool::connect(&database_url).await.unwrap()
}

// ドキュメントの情報を取得する
pub async fn get_web_page(
    pool: &PgPool,
    id: i64,
    page_path: &str,
) -> Result<WebPages, sqlx::Error> {
    let page = sqlx::query_as::<_, WebPages>(r##"SELECT app_id, page_path, file_path FROM public."web_pages" WHERE app_id=$1 and page_path=$2"##)
        .bind(id)
        .bind(page_path)
        .fetch_one(pool)
        .await?;
    println!("{:?}", &page);
    Ok(page)
}

/**
ページの親を指定して、子を取り出す
その後、フロント側で処理する形に変形する
一番上の階層のときだけ使用する
 */

pub async fn get_page_structure(pool: &PgPool, app_id: i64, parent_path:String, depth: i32) -> Vec<HierarchyTS>{
    if depth != 1{
        panic!("depth =1 以外のデータが投入された");
    }
    let pages = sqlx::query_as::<_, Hierarchy>(
        r##"SELECT id, app_id, child_path, depth FROM public."page_hierarchy" 
        WHERE app_id=$1 AND parent_path=$2 AND depth=$3"##,
    )
    .bind(app_id)
    .bind(parent_path)
    .bind(depth+1)
    .fetch_all(pool)
    .await
    .unwrap();

    println!("{:?}",&pages);
    pages.into_iter().map(|x| x.into_ts()).collect()

}

/**
ページの親を指定して、子を取り出す
その後、フロント側で処理する形に変形する
 */

pub async fn get_page_structure_from_id(pool: &PgPool, id:i64) -> Vec<HierarchyTS>{
    let pages = sqlx::query_as::<_, Hierarchy>(
        r##"  WITH X AS (SELECT * FROM public."page_hierarchy" WHERE id =$1 )
        SELECT ph.id, ph.app_id, ph.child_path, ph.depth FROM public."page_hierarchy" ph ,X
        WHERE ph.app_id=X.app_id AND ph.parent_path=X.child_path AND ph.depth=X.depth + 1;"##,
    )
    .bind(id)
    .fetch_all(pool)
    .await
    .unwrap();
    pages.into_iter().map(|x| x.into_ts()).collect()
    

}


// get_page_path で使うためのストラクト
#[derive(sqlx::FromRow)]
struct ChildPath{
    child_path: String,
}

// もらったpage_hierarchy のSerial で、祖先までのパスを取得する。
pub async fn get_page_path(pool: &PgPool, path_id: i64) -> String{
    let mut url = PathBuf::from("");

    // path_id の親すべて(自身も含む)を列挙するSQL
    let pages= sqlx::query_as::<_,ChildPath>(r##" WITH RECURSIVE X( parent_path,child_path,depth) AS 
    (SELECT ph.parent_path,ph.child_path, ph.depth FROM public."page_hierarchy"  ph WHERE ph.id = $1
    union  all
    select ph.parent_path, ph.child_path, ph.depth from X,public."page_hierarchy"  ph
    where X.parent_path = ph.child_path AND X.parent_path != X.child_path)
    SELECT child_path FROM X order by depth;
    "##).bind(path_id)
    .fetch_all(pool).await.unwrap();

    for row in pages.iter(){
        url.push(&row.child_path)
    }

    url.into_os_string().into_string().unwrap()
}


// ドキュメントを追加する。 page_id はSerial で勝手に振られるので、適当な値を入れておく。
pub async fn add_web_page(pool: &PgPool, page: WebPageInfo) -> Result<(), sqlx::Error> {
    if get_web_page(pool, page.app_id, &page.page_path)
        .await
        .is_ok()
    {
        return Err(sqlx::Error::RowNotFound);
    }

    let file_path = &page.create_file_path();
    let page_path = &page.get_page_path();

    // transaction start 
    let mut tx = pool.begin().await?;

    // page_path をpage_hierarchy 用に分解
    let paths:Vec<&str> = page_path.split('/').collect();
    let l = &paths.len();
    
    // page_hierarchy にデータを登録する
    for (i, _path) in paths.clone().into_iter().enumerate(){
        let j = i as i32;
        if i+1 != *l{
            let res = sqlx::query!(r##"
            SELECT app_id FROM public."page_hierarchy" WHERE app_id = $1 AND parent_path = $2 AND child_path = $3 AND depth = $4
            "##,
            page.app_id,
            &paths[i],
            &paths[i+1],
            j+2
            ).fetch_one(&mut tx).await;
        match res {
            Ok(_) => {
                // すでに存在するデータなので何もしない
            }
            Err(_) =>{
                // データがないので、page_hierarchy にデータ追加する
                let j = i as i32;
                let _q = sqlx::query!(r##"
                INSERT INTO public."page_hierarchy"  (app_id, parent_path, child_path, depth) VALUES ($1,$2, $3, $4)
                "##,
            page.app_id,
            &paths[i],
            &paths[i+1],
            j +2
        ).execute(&mut tx)
        .await?;
            }
        }
        }
    }

    // web_pages にデータ追加
    let p = sqlx::query!(
        r##" INSERT INTO public."web_pages" (app_id, page_path, file_path) VALUES ($1, $2, $3)"##,
        page.app_id,
        &page_path,
        &file_path
    )
    .execute(&mut tx)
    .await;

    

    // 処理がうまく行けば transaction をコミットする。
    match p {
        Ok(_) => {
            let page_data = page.page_data.as_ref().unwrap();
            if let Err(err) =fs::write(&file_path, page_data){
                tx.rollback().await?;
                Err(sqlx::Error::Io(err))
                
            }
            else{
                tx.commit().await?;
                Ok(())
            }
        }
        Err(err) => {
            tx.rollback().await?;
            Err(err)
        }
    }
}


#[cfg(test)]
mod tests{
    use super::*;

    // 所定のファイル名を作成できるかテストする。
    // どのディレクトリに作られているか(current_dir/md/)はテストしない
    #[test]
    fn file_path_test() {
        let info = WebPageInfo {
            app_id: 100,
            page_path: "doc/hoge/test.md".to_string(),
            page_data: Some(String::from(
                r##"# hoge  
            ## hanage"##,
            )),
        };
        let file_path = info.create_file_path();
        assert_eq!("100@doc@hoge@test.md", Path::new(&file_path).file_name().unwrap().to_str().unwrap())
    }

    // md/test.md に# test desu という内容のファイルがあるときに、呼び出せるかテスト。
    #[test]
    fn get_markdown_test() {
        let test = String::from("# test desu");

        let info = WebPageInfo {
            app_id: 1,
            page_path: "test.md".to_string(),
            page_data: None,
        };

        let markdown_text = info.get_markdown();

        assert_eq!(test, markdown_text.unwrap())
    }

    // 階層構造からファイルパスを組み立てられるかテスト。 自動化できてはない。
    #[tokio::test]
    async fn get_page_path_test(){
        let conn = get_conn().await;
        let p = get_page_path(&conn, 6).await;
        assert_eq!(p, "app/hoge.md".to_string())
    }
}