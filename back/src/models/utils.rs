use crate::models::schemas::*;
use dotenv::dotenv;
// use serde::{de::value::StrDeserializer, Deserialize, Serialize};
use sqlx::{postgres::PgPool};
use std::{env, fs, io, path::Path};

/**
データから、PageStructure を構成する。
PageHierarchy のapp_id が複数ある時はパニックする。
再帰関数にすればおっけー
Todo:error 出てるので直す
 */
// pub fn correct_paeg_structure(
//     pages: Vec<PageHierarchy>,
//     app_name: String,
// ) -> PageStructure<'static> {
//     let i = get_app_id(&pages);

//     let ancients = pages
//         .iter()
//         .filter(|p| p.parent_path == app_name && p.hierarchy_difference == 1);

//     let mut p_s = Vec::new();
//     for a in ancients {
//         let mut p_s_c = Vec::new();
//         p_s.push(&PageStructure::Data {
//             name: a.child_path,
//             children: if a.child_path.split('.').collect::<String>().len() == 1 {
//                 Box::new(vec![&PageStructure::None])
//             } else {
//                 // 再帰始まり
//                 let childs = pages
//                     .iter()
//                     .filter(|p| p.parent_path == a.child_path && p.hierarchy_difference == 1);
//                 for c in childs {
//                     let mut p_s_gc = Vec::new();
//                     p_s_c.push(&PageStructure::Data {
//                         name: c.child_path,
//                         children: if c.child_path.split('.').collect::<String>().len() == 1 {
//                             Box::new(vec![&PageStructure::None])
//                         } else {
//                             let g_childs = pages.iter().filter(|p| {
//                                 p.parent_path == c.child_path && p.hierarchy_difference == 1
//                             });
//                             for g in g_childs {
//                                 p_s_gc.push(&PageStructure::Data {
//                                     name: g.child_path,
//                                     children: if g.child_path.split('.').collect::<String>().len()
//                                         == 1
//                                     {
//                                         Box::new(vec![&PageStructure::None])
//                                     } else {
//                                         // 再帰終わり
//                                         Box::new(vec![&PageStructure::None])
//                                     },
//                                 })
//                             }
//                             Box::new(p_s_gc)
//                         },
//                     });
//                 }
//                 Box::new(p_s_c)
//             },
//         })
//     }
//     PageStructure::Data {
//         name: app_name,
//         children: Box::new(p_s),
//     }
// }

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

    Ok(page)
}

/**
ページの階層構造を取り出してJSON型で返す関数
 */

pub async fn get_page_structure(pool: &PgPool, app_id: i64) -> String {
    // let pages = sqlx::query_as::<_, PageHierarchy>(
    //     r##"SELECT * FROM public."page_hierarchy" WHERE app_id=$1"##,
    // )
    // .bind(app_id)
    // .fetch_all(pool)
    // .await
    // .unwrap();

    let pages = sqlx::query_as::<_, WebPages>(
        r##"SELECT app_id, page_path, file_path FROM public."web_pages" WHERE app_id=$1"##,
    )
    .bind(app_id)
    .fetch_all(pool)
    .await
    .unwrap();

    let app_name = "".to_string();
    // let data = correct_paeg_structure(pages, app_name);
    let data = "".to_string();
    serde_json::to_string(&data).unwrap()
}

pub fn get_ancient(pages: Vec<PageHierarchy>) {}

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

    println!("add_web_page = {:?}", page);

    let p = sqlx::query!(
        r##" INSERT INTO public."web_pages" (app_id, page_path, file_path) VALUES ($1, $2, $3)"##,
        page.app_id,
        page_path,
        file_path
    )
    .execute(pool)
    .await;

    match p {
        Ok(_) => {
            let page_data = page.page_data.as_ref().unwrap();
            fs::write(&file_path, page_data).unwrap();
            Ok(())
        }
        Err(err) => Err(err),
    }
}
