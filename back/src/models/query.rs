use crate::models::{requests::*, schemas::*};
use dotenv::dotenv;
use sqlx::{postgres::PgPool, Row};
use std::{env, io};

/// page_hierarchy の一番上のデータのdepth を決める定数
pub const HIERARCHY_TOP_NUMBER: i32 = 0;
/// page_hierarchy  の一番上のデータのparent を決める定数
pub const HIERARCHY_TOP_PARENT_ID: i64 = -99;

// ドキュメントの情報を取得する
pub async fn get_web_page(
    pool: &PgPool,
    id: i64,
    page_path: &str,
) -> Result<WebPages, sqlx::Error> {
    println!("In get_web_page id={}, page_path={}", id, page_path);
    let page = sqlx::query_as::<_, WebPages>(r##"SELECT app_id, page_path, file_path FROM public."web_pages" WHERE app_id=$1 and page_path=$2"##)
        .bind(id)
        .bind(page_path)
        .fetch_one(pool)
        .await?;
    Ok(page)
}

/// page_hierarchy からdepth = HIERARCHY_TOP_NUMBER + 1 のデータを取得する
pub async fn get_page_structure(pool: &PgPool, app_id: i64) -> Vec<HierarchyTS> {
    let pages = sqlx::query_as::<_, Hierarchy>(
        r##"SELECT id, app_id, child as child_path, depth FROM public."page_hierarchy" 
        WHERE app_id=$1 AND depth=$2"##,
    )
    .bind(app_id)
    .bind(HIERARCHY_TOP_NUMBER + 1)
    .fetch_all(pool)
    .await
    .unwrap();

    pages.into_iter().map(|x| x.into_ts()).collect()
}

/// ページの親を(postgressqlのserialで)指定して、子を取り出す
/// その後、フロント側で処理する形に変形する
/// id -> page_hierarchy.id
pub async fn get_page_structure_from_id(pool: &PgPool, id: i64) -> Vec<HierarchyTS> {
    let pages = sqlx::query_as::<_, Hierarchy>(
        r##"  WITH X AS (SELECT * FROM public."page_hierarchy" WHERE id =$1 )
        SELECT ph.id, ph.app_id, ph.child as child_path, ph.depth FROM public."page_hierarchy" ph ,X
        WHERE ph.app_id=X.app_id AND ph.parent=X.id AND ph.depth=X.depth + 1;"##,
    )
    .bind(id)
    .fetch_all(pool)
    .await
    .unwrap();
    pages.into_iter().map(|x| x.into_ts()).collect()
}

/// get_page_path で使うためのストラクト
#[derive(sqlx::FromRow, Debug)]
struct ChildPath {
    child: String,
}

/// もらったpage_hierarchy のSerial で、祖先までのページパス(app/hoge/hogege.md など)を取得する。
/// パスは、'/'区切り
pub async fn get_page_path(pool: &PgPool, path_id: i64) -> String {
    // path_id の親を特定するのに使う
    let app_id = sqlx::query!(
        r##"SELECT app_id FROM public."page_hierarchy" WHERE id = $1 "##,
        path_id
    )
    .fetch_one(pool)
    .await
    .unwrap();
    let mut url = String::from("");

    // path_id の親すべて(自身も含む)を列挙するSQL
    let pages = sqlx::query_as::<_, ChildPath>(
        r##" WITH RECURSIVE X(id, parent,child,depth) AS
    (SELECT ph.id,ph.parent,ph.child, ph.depth FROM public."page_hierarchy"  ph WHERE ph.id = $1 AND ph.app_id=$2
    union  all
    select ph.id,ph.parent, ph.child, ph.depth from X,public."page_hierarchy"  ph
    where X.parent = ph.id AND ph.app_id= $2)
    SELECT child FROM X order by depth;
    "##,
    )
    .bind(path_id)
    .bind(app_id.app_id)
    .fetch_all(pool)
    .await
    .unwrap();

    // app/hoge/abc.md/ の形のString を作る
    for child_path in pages.iter() {
        url.push_str(&child_path.child);
        url.push('/');
    }
    // 最後の/ は不要なので削除する
    let _ = url.remove(url.len() - 1);
    url
}

/// delete_pagesで使うためのストラクト
#[derive(sqlx::FromRow)]
struct HierarchyId {
    id: i64,
    child_path: String,
}

/// もらったparent_pathの子どものpage_hierarchy, web_pagesのデータ、マークダウンのファイルを削除する  
/// id : page_hierarchy のpkey
pub async fn delete_pages(pool: &PgPool, id: i64) -> Result<(), sqlx::Error> {
    dotenv().ok();
    // delete_page の時に使うurl
    let url = env::var("FILE_SERVER_URL").expect("FILE_SERVER_URL must be set");

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
        WITH RECURSIVE X AS 
(SELECT * FROM public."page_hierarchy"  ph 
WHERE ph.id = $1 AND ph.app_id = $2
union  all
select ph.id, ph.app_id, ph.parent,ph.child, ph.depth, ph.created_at, ph.updated_at from X,public."page_hierarchy"  ph
where ph.parent = X.id AND X.depth > $3  AND ph.app_id=$2)
SELECT id, child as child_path FROM X;
"##,
    )
    .bind(id)
    .bind(app_id.app_id)
    .bind(HIERARCHY_TOP_NUMBER)
    .fetch_all(&mut tx)
    .await?;

    // page_hierarchy から削除
    if let Err(err) = sqlx::query(
        r##" 
        WITH RECURSIVE X AS 
(SELECT * FROM public."page_hierarchy"  ph 
WHERE ph.id = $1 AND ph.app_id = $2
union  all
select ph.id, ph.app_id, ph.parent,ph.child, ph.depth, ph.created_at, ph.updated_at from X,public."page_hierarchy"  ph
where ph.parent = X.id AND X.depth > $3  AND ph.app_id=$2)
DELETE FROM public."page_hierarchy" WHERE id in( SELECT id FROM X);
"##,
    )
    .bind(id)
    .bind(app_id.app_id)
    .bind(HIERARCHY_TOP_NUMBER)
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
        if let Err(e) = delete_markdown(&url, &file_path).await {
            println!("in delete_page, error occured, {}", e.to_string());
            let err = io::Error::new(io::ErrorKind::NotFound, e.to_string());
            tx.rollback().await?;
            return Err(sqlx::Error::Io(err));
        };
    }
    // web_pages から削除終わり

    // トランザクション終わり
    Ok(tx.commit().await?)
}

/// fileにデータを書きこむ
async fn write_file(url: String, file_path: &str, page_data: &str) -> Result<(), sqlx::Error> {
    if let Err(e) = post_markdown(url, file_path, page_data).await {
        println!("in weite_file, error occured, {}", e.to_string());
        let err = io::Error::new(io::ErrorKind::NotFound, e.to_string());
        return Err(sqlx::Error::Io(err));
    }
    Ok(())
}

/// fileのデータをアップデートする
async fn update_file(url: String, file_path: &str, page_data: &str) -> Result<(), sqlx::Error> {
    if let Err(e) = put_markdown(url, file_path, page_data).await {
        println!("in update_file, error occured, {}", e.to_string());
        let err = io::Error::new(io::ErrorKind::NotFound, e.to_string());
        return Err(sqlx::Error::Io(err));
    }
    Ok(())
}

#[derive(sqlx::FromRow)]
struct Id {
    id: i64,
}

/// ドキュメントを追加する。 page_id はSerial で勝手に振られるので、適当な値を入れておく。
/// 既にデータが存在する時はupdate,ファイル更新だけする。
pub async fn add_web_page(pool: &PgPool, page: WebPageInfo) -> Result<(), sqlx::Error> {
    dotenv().ok();
    let url = env::var("FILE_SERVER_URL").expect("FILE_SERVER_URL must be set");
    let file_path = &page.create_file_path();
    let page_path = &page.get_page_path();
    println!("regitering page_path = {}", page_path);

    if get_web_page(pool, page.app_id, &page.page_path)
        .await
        .is_ok()
    {
        // 既にデータが存在するので、ファイルを更新する
        return update_file(url, file_path, page.page_data.as_ref().unwrap()).await;
    }

    // transaction start
    let mut tx = pool.begin().await?;

    // page_hierarchyに登録するデータの入れ物
    let mut res: Vec<PageDecomposition> = Vec::new();

    // page_path をpage_hierarchy 用に分解
    let page_elements = page_path.split('/');

    // page_hierarchy にデータを登録する
    // ToDo: res を関数内部から削除する
    // ToDo: 何をしているのか日本語で説明を書く。
    // ToDo: 出来るならここのfor 文を関数に切り出す。
    // 親の情報でpage_hierarchyからSELECT してみる
    // -> 出来る->何もしない , 出来ない-> INSERT

    for (i, page_element) in page_elements.enumerate() {
        if i == 0 {
            let first_row = sqlx::query!(
                r##"
            SELECT id  FROM public."page_hierarchy" WHERE app_id=$1 AND parent=$2 AND depth = $3
            "##,
                page.app_id,
                HIERARCHY_TOP_PARENT_ID,
                HIERARCHY_TOP_NUMBER,
            )
            .fetch_one(pool)
            .await
            .unwrap();
            res.push(PageDecomposition {
                id: first_row.id,
                parent: HIERARCHY_TOP_PARENT_ID,
                child: page_element.to_string(),
            });
        } else {
            let depth = HIERARCHY_TOP_NUMBER + i as i32;
            // page_hierarchy からSELECT してみる
            let parent_row = sqlx::query!(
                        r##"
                    SELECT id  FROM public."page_hierarchy" WHERE app_id=$1 AND parent=$2 AND child = $3 AND depth = $4 
                    "##,
                        page.app_id,
                        res[i - 1].id,
                        page_element,
                        depth
                    )
                    .fetch_one(pool)
                    .await;
            match parent_row {
                Ok(selected_row) => {
                    // すでに存在するデータなのでINSERT はしない
                    res.push(PageDecomposition {
                        id: selected_row.id,
                        parent: res[i - 1].id,
                        child: page_element.to_string(),
                    });
                }
                Err(_) => {
                    // データが存在しないのでINSERT
                    let parent_id = sqlx::query_as::<_,Id>(r##"
                    INSERT INTO public."page_hierarchy"  (app_id, parent, child, depth) VALUES ($1,$2, $3, $4) RETURNING id
                    "##)
                    .bind(page.app_id)
                    .bind(res[i-1].id)
                    .bind(page_element.to_string())
                    .bind(depth)
                    .fetch_one(&mut tx)
                    .await;
                    match parent_id {
                        Ok(got_id) => {
                            res.push(PageDecomposition {
                                id: got_id.id,
                                parent: res[i - 1].id,
                                child: page_element.to_string(),
                            });
                        }
                        Err(e) => {
                            tx.rollback().await?;
                            println!("{:?}", &res);
                            return Err(e);
                        }
                    }
                }
            }
        }
    }
    println!(
        "In add_web_page, res(vector of PageDecompotion) = {:?}",
        res
    );

    // web_pages にデータ追加
    let added_page = sqlx::query!(
        r##" INSERT INTO public."web_pages" (app_id, page_path, file_path) VALUES ($1, $2, $3)"##,
        page.app_id,
        &page_path,
        &file_path
    )
    .execute(&mut tx)
    .await;

    // dbの処理が上手くいき、ファイルの更新が上手くいったらtransaction をコミットする。
    match added_page {
        Ok(_) => {
            let page_data = page.page_data.as_ref().unwrap();
            let url = env::var("FILE_SERVER_URL").expect("FILE_SERVER_URL must be set");
            if let Err(err) = write_file(url, &file_path, page_data).await {
                tx.rollback().await?;
                Err(err)
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

/// web_pages からデータを検索する為の情報を、page_hiearachyから取得する。
/// returnのi64 -> app_id, String -> page_path
pub async fn get_web_page_info(
    pool: &PgPool,
    hierarchy_id: i64,
) -> Result<(i64, String), sqlx::Error> {
    let row = sqlx::query!(
        r##"
            SELECT id, app_id, child  FROM public."page_hierarchy" WHERE id = $1 
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
    use dotenv::dotenv;
    use sqlx::postgres::PgPoolOptions;
    use std::{env, path::Path, time::Duration};

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

    #[tokio::test]
    async fn rows_exist_test() {
        dotenv().ok();
        // db connection string
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        //db connection pool
        let pool = PgPoolOptions::new()
            .max_connections(3)
            .connect_timeout(Duration::from_secs(3))
            .connect(&database_url)
            .await
            .expect("can connect to database");
        let rows_exist = sqlx::query(
            r##"
                SELECT EXISTS ( SELECT *  FROM public."web_pages" WHERE app_id = $1) 
                "##,
        )
        .bind(-9999 as i64)
        .fetch_one(&pool)
        .await;
        match rows_exist {
            Ok(row) => {
                let is_exist: bool = row.get("exists");
                assert_eq!(is_exist, false)
            }
            Err(e) => {
                println!("{}", e);
                assert!(false)
            }
        }
    }

    // md/test.md に# test desu という内容のファイルがあるときに、呼び出せるかテスト。
    // #[test]
    // fn get_markdown_test() {
    //     let test = String::from("# test desu");

    //     let info = WebPageInfo {
    //         app_id: 1,
    //         page_path: "test.md".to_string(),
    //         page_data: None,
    //     };

    //     let markdown_text = info.get_markdown();

    //     assert_eq!(test, markdown_text.unwrap())
    // }
}
