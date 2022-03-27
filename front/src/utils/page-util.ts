import axios from "axios"
// backend側にデータを保存してもらい、成功したらナビゲーションバーにパスを追加する
export function Save(app_id: number, page_path: string, data: string): void {
    console.log("post to /add");
    console.log("app_id : " + app_id);
    console.log("page_path : " + page_path);
    console.log("data : " + data);
    axios.post("/add", {
        app_id: app_id,
        page_path: page_path,
        page_data: data,
    })
        .then(function (response) {
            console.log("Save Markdown ");
            console.log(response);
            AddPathData(app_id, page_path);
        })
        .catch(function (response) {
            alert(response);
            console.log(response);
        });

}

export function AddPathData(app_id: number, page_path: string) {
    console.log("Add new page path to Navigation Bar ");
}

export function IsExistPage(app_id: number, page_path: string): boolean {
    let is_exist = false;
    console.log("Calling IsExistPage app_id = " +app_id + "page_path = " + page_path  );
    axios.post("/check", {
        app_id: app_id,
        page_path: page_path,
    })
        .then(function (response) {
            is_exist = response.data === true;
        })
        .catch(function (response) {
            console.log(response);
            is_exist = true;
        });
    return is_exist;
}