use back::models::*;

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
    assert_eq!("100@doc@hoge@test.md", file_path)
}

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

#[tokio::test]
async fn path_structure_test() {
    let pool = get_conn().await;
    let a = get_paeg_structure(&pool, 1).await.unwrap();
    assert_eq!(a, ())
}
