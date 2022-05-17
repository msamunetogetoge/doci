use crate::models::schemas::*;
use dotenv::dotenv;
// use serde::{de::value::StrDeserializer, Deserialize, Serialize};
use sqlx::{postgres::PgPool, Row};
use std::{
    env, fs, io,
    path::{Path, PathBuf},
};

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

pub async fn get_page_structure(
    pool: &PgPool,
    app_id: i64,
    parent_path: String,
    depth: i32,
) -> Vec<HierarchyTS> {
    if depth != 1 {
        panic!("depth =1 以外のデータが投入された");
    }
    let pages = sqlx::query_as::<_, Hierarchy>(
        r##"SELECT id, app_id, child_path, depth FROM public."page_hierarchy" 
        WHERE app_id=$1 AND parent_path=$2 AND depth=$3"##,
    )
    .bind(app_id)
    .bind(parent_path)
    .bind(depth + 1)
    .fetch_all(pool)
    .await
    .unwrap();

    println!("{:?}", &pages);
    pages.into_iter().map(|x| x.into_ts()).collect()
}

/**
ページの親を(postgressqlのserialで)指定して、子を取り出す
その後、フロント側で処理する形に変形する
 */

pub async fn get_page_structure_from_id(pool: &PgPool, id: i64) -> Vec<HierarchyTS> {
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
struct ChildPath {
    child_path: String,
}

// もらったpage_hierarchy のSerial で、祖先までのページパス(app/hoge/hogege.md など)を取得する。
// パスは、'/'区切り
pub async fn get_page_path(pool: &PgPool, path_id: i64) -> String {
    // let mut url = PathBuf::from("");
    let mut url = String::from("");

    // path_id の親すべて(自身も含む)を列挙するSQL
    let pages = sqlx::query_as::<_, ChildPath>(
        r##" WITH RECURSIVE X( parent_path,child_path,depth) AS 
    (SELECT ph.parent_path,ph.child_path, ph.depth FROM public."page_hierarchy"  ph WHERE ph.id = $1
    union  all
    select ph.parent_path, ph.child_path, ph.depth from X,public."page_hierarchy"  ph
    where X.parent_path = ph.child_path AND X.parent_path != X.child_path)
    SELECT child_path FROM X order by depth;
    "##,
    )
    .bind(path_id)
    .fetch_all(pool)
    .await
    .unwrap();

    // app/hoge/abc.md/ の形のString を作る
    for row in pages.iter() {
        url.push_str(&row.child_path);
        url.push('/');
    }
    // 最後の/ は不要なので削除する
    let _ = url.remove(url.len() - 1);

    url
}

// delete_pagesで使うためのストラクト
#[derive(sqlx::FromRow)]
struct HierarchyId {
    id: i64,
    child_path: String,
}

// もらったparent_pathの子どものpage_hierarchy, web_pagesのデータ、マークダウンのファイルを削除する

pub async fn delete_pages(pool: &PgPool, id: i64) -> Result<(), sqlx::Error> {
    let app_id = sqlx::query!(
        r##"SELECT app_id FROM public."page_hierarchy" WHERE id = $1 "##,
        id
    )
    .fetch_one(pool)
    .await?;

    // トランザクションスタート
    let mut tx = pool.begin().await?;

    // web_pages からデータを削除するときの、page_path作成のためにデータを取得しておく
    let pages = sqlx::query_as::<_, HierarchyId>(
        r##"
    WITH RECURSIVE X(id, parent_path,child_path) AS 
    (SELECT ph.id,ph.parent_path,ph.child_path FROM public."page_hierarchy"  ph 
WHERE ph.id = $1
    union  all
    select ph.id, ph.parent_path, ph.child_path from X,public."page_hierarchy"  ph
    where ph.parent_path = X.child_path AND X.parent_path != X.child_path)
    SELECT id,child_path FROM X;
    "##,
    )
    .bind(id)
    .fetch_all(&mut tx)
    .await?;

    // page_hierarchy から削除
    if let Err(err) = sqlx::query(
        r##" 
    WITH RECURSIVE X(id, parent_path,child_path) AS 
    (SELECT ph.id,ph.parent_path,ph.child_path FROM public."page_hierarchy"  ph 
WHERE ph.id = $1
    union  all
    select ph.id, ph.parent_path, ph.child_path from X,public."page_hierarchy"  ph
    where ph.parent_path = X.child_path AND X.parent_path != X.child_path)
    DELETE FROM public."page_hierarchy" WHERE id in( SELECT id FROM X);
    "##,
    )
    .bind(id)
    .fetch_all(&mut tx)
    .await
    {
        tx.rollback().await?;
        return Err(err);
    };

    // web_pagesから削除スタート
    let mut delete_pages: Vec<String> = vec![];

    for page in pages.into_iter() {
        // pages からデータ(*.md がchild_path にあるデータ)を抽出
        if page.child_path.contains(".md") {
            // 取得したデータからページパスを作成
            delete_pages.push(get_page_path(pool, page.id).await);
        }
    }

    // web_pages から削除
    for page_path in delete_pages {
        println!("in delete_pages, pages in web_page is  {}", page_path);

        // ファイルを削除するためにデータを取得しておく
        let row = sqlx::query(
            r##" SELECT file_path  FROM public."web_pages" WHERE app_id = $1 AND page_path =$2"##,
        )
        .bind(app_id.app_id)
        .bind(&page_path)
        .fetch_one(&mut tx)
        .await?;

        if let Err(err) =
            sqlx::query(r##" DELETE FROM public."web_pages" WHERE app_id = $1 AND page_path =$2"##)
                .bind(app_id.app_id)
                .bind(&page_path)
                .fetch_all(&mut tx)
                .await
        {
            tx.rollback().await?;
            return Err(err);
        }

        // ファイルの削除
        let file_path: String = row.get("file_path");
        if let Err(e) = fs::remove_file(file_path) {
            tx.rollback().await?;
            return Err(sqlx::Error::Io(e));
        };
    }
    // web_pages から削除終わり

    // トランザクション終わり
    Ok(tx.commit().await?)
}

// ドキュメントを追加する。 page_id はSerial で勝手に振られるので、適当な値を入れておく。
// 既にデータが存在する時はupdateファイル更新だけする。
pub async fn add_web_page(pool: &PgPool, page: WebPageInfo) -> Result<(), sqlx::Error> {
    let file_path = &page.create_file_path();
    let page_path = &page.get_page_path();

    if let Ok(_) = get_web_page(pool, page.app_id, &page.page_path).await {
        // 既にデータが存在するので、ファイルを更新する
        let page_data = page.page_data.as_ref().unwrap();
        if let Err(err) = fs::write(&file_path, page_data) {
            return Err(sqlx::Error::Io(err));
        } else {
            return Ok(());
        }
    }

    // transaction start
    let mut tx = pool.begin().await?;

    // page_path をpage_hierarchy 用に分解
    let paths: Vec<&str> = page_path.split('/').collect();
    let l = &paths.len();

    // page_hierarchy にデータを登録する
    for (i, _path) in paths.clone().into_iter().enumerate() {
        let j = i as i32;
        if i + 1 != *l {
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
                Err(_) => {
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
            if let Err(err) = fs::write(&file_path, page_data) {
                tx.rollback().await?;
                Err(sqlx::Error::Io(err))
            } else {
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

// web_pages からデータを検索する為の情報を、page_hiearachyから取得する。
// returnのi64 -> app_id, String -> page_path
pub async fn get_web_page_info(
    pool: &PgPool,
    hierarchy_id: i64,
) -> Result<(i64, String), sqlx::Error> {
    let row = sqlx::query!(
        r##"
            SELECT id, app_id, child_path  FROM public."page_hierarchy" WHERE id = $1 
            "##,
        hierarchy_id
    )
    .fetch_one(pool)
    .await?;
    let page_path = get_page_path(pool, row.id).await;
    Ok((row.app_id, page_path))
}

#[cfg(test)]
mod tests {
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
        assert_eq!(
            "100@doc@hoge@test.md",
            Path::new(&file_path).file_name().unwrap().to_str().unwrap()
        )
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
    async fn get_page_path_test() {
        let conn = get_conn().await;
        let p = get_page_path(&conn, 6).await;
        assert_eq!(p, "app/hoge.md".to_string())
    }
}
