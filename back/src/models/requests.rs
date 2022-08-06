use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Post {
    file_name: String,
    content: String,
}

#[derive(Serialize, Deserialize)]
struct Put {
    file_name: String,
    content: String,
}

#[derive(Serialize, Deserialize)]
struct Delete {
    file_name: String,
}

#[derive(Serialize, Deserialize)]
struct Get {
    file_name: String,
}

#[derive(Serialize, Deserialize)]
struct GetResponse {
    content: String,
}

pub async fn post_markdown(
    url: String,
    file_name: &str,
    content: &str,
) -> Result<(), reqwest::Error> {
    let post_info = Post {
        file_name: file_name.to_string(),
        content: content.to_string(),
    };
    let _response = reqwest::Client::new()
        .post(url)
        .json(&post_info)
        .send()
        .await?;

    Ok(())
}

pub async fn get_markdown_from_gcs(url: String, file_name: &str) -> Result<String, reqwest::Error> {
    let get_url = url + "/" + file_name;
    let response: GetResponse = reqwest::Client::new()
        .get(get_url)
        .send()
        .await?
        .json()
        .await?;
    Ok(response.content)
}

pub async fn put_markdown(
    url: String,
    file_name: &str,
    content: &str,
) -> Result<(), reqwest::Error> {
    let put_info = Put {
        file_name: file_name.to_string(),
        content: content.to_string(),
    };
    let _response = reqwest::Client::new()
        .put(url)
        .json(&put_info)
        .send()
        .await?;

    Ok(())
}

pub async fn delete_markdown(url: &str, file_name: &str) -> Result<(), reqwest::Error> {
    let delete_info = Delete {
        file_name: file_name.to_string(),
    };
    let _response = reqwest::Client::new()
        .delete(url)
        .json(&delete_info)
        .send()
        .await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use dotenv::dotenv;
    use http::StatusCode;
    use std::env;

    use super::*;

    /// post_markdownが動くかテスト
    #[tokio::test]
    async fn post_path_test() {
        let url = "https://httpbin.org/post".to_string();
        match post_markdown(url, "test", "test_content").await {
            Ok(_) => {}
            Err(e) => panic!("{}", e),
        }
    }

    /// ファイルの読み書きを行うurlにアクセスして404が返ってこないかテスト
    #[tokio::test]
    async fn file_server_connect_test() {
        dotenv().ok();
        let url = env::var("FILE_SERVER_URL").expect("FILE_SERVER_URL must be set");
        let response = reqwest::Client::new().get(url).send().await.unwrap();
        assert!(response.status() != StatusCode::NOT_FOUND)
    }
}
