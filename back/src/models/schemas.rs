use serde::{Deserialize, Serialize};
use std::{
    env, fs, io,
    path::{Path, PathBuf},
};

/// users table
#[derive(sqlx::FromRow)]
pub struct Users {
    pub id: i64,
    pub name: String,
    pub password: String,
    pub mailaddress: String,
}

#[derive(Serialize, Debug)]
pub enum Children {
    EmptyChild,
    Child(Box<Hierarchy>),
}

#[derive(sqlx::FromRow, Serialize, Debug)]
pub struct Hierarchy {
    pub id: i64,
    pub app_id: i64,
    pub child_path: String,
    pub depth: i32,
}

#[derive(sqlx::FromRow, Serialize, Debug)]
pub struct HierarchyTS {
    pub id: i64,
    pub app_id: i64,
    pub name: String,
    pub depth: i32,
    pub children: Option<Vec<Children>>,
}
impl Hierarchy {
    pub fn into_ts(self) -> HierarchyTS {
        let path = Path::new(&self.child_path);
        if path.extension() != None && path.extension().unwrap() == "md" {
            HierarchyTS {
                id: self.id,
                app_id: self.app_id,
                name: self.child_path,
                depth: self.depth,
                children: None,
            }
        } else {
            HierarchyTS {
                id: self.id,
                app_id: self.app_id,
                name: self.child_path,
                depth: self.depth,
                children: Some(Vec::new()),
            }
        }
    }
}

/**
web_pages table
file_pathはmd@app_id@hoge@hige@huga.md の形に成形して格納する(/ -> @ の置換)
*/
#[derive(sqlx::FromRow, Debug)]
pub struct WebPages {
    pub app_id: i64,
    pub page_path: String,
    pub file_path: String,
}

// post from client, document infomation
#[derive(Serialize, Deserialize, Debug)]
pub struct WebPageInfo {
    pub app_id: i64,
    pub page_path: String,
    pub page_data: Option<String>,
}

impl WebPageInfo {
    /**
    アプリidとページパスの情報からmdの格納先情報を作成する
    app_id@hoge@hige@huga.md の形に成形
    /hoge/hige/huga.md  の形でもらう
    */
    pub fn create_file_path(&self) -> String {
        let folder_path = env::current_dir().unwrap().join(Path::new("md"));
        let mut file_path = self.app_id.to_string();

        let split = self.page_path.split('/');

        for path in split {
            if path == "".to_string() {
                continue;
            }
            file_path += "@";
            file_path += path;
        }
        let mut _file_path = PathBuf::from(file_path);
        _file_path.set_extension("md");
        file_path = folder_path
            .join(_file_path.as_path())
            .to_str()
            .unwrap()
            .to_string();

        file_path
    }

    /**
    pagepath に .md が付いていなかったら追加して返す
    */
    pub fn get_page_path(&self) -> String {
        let mut page_path = PathBuf::new();
        page_path.push(&self.page_path);
        page_path.set_extension("md");
        return page_path.as_path().to_str().unwrap().to_string();
    }

    /**
    markdown ファイルを取得する
    */
    pub fn get_markdown(&self) -> Result<String, io::Error> {
        let folder_path = env::current_dir().unwrap().join(Path::new("md"));

        let file_path = folder_path.join(self.create_file_path());

        fs::read_to_string(file_path)
    }

    /**
    マークダウンを上書き保存する
    */
    pub fn edit_web_page(&self) -> io::Result<()> {
        let folder_path = env::current_dir().unwrap().join(Path::new("md"));

        let file_path = folder_path.join(self.create_file_path());
        fs::write(file_path, self.page_data.as_ref().unwrap())
    }
}

/**
page_hierarchy table
*/
#[derive(sqlx::FromRow)]
pub struct PageHierarchy {
    pub id: Option<i64>,
    pub app_id: i64,
    pub parent_path: String,
    pub child_path: String,
    pub depth: i32,
}
