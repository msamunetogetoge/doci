import axios from "axios"
// backend側にデータを保存してもらい、成功したらナビゲーションバーにパスを追加する
export function Save(app_id: number, page_path: string, data: string): void {
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
            console.log(response);
        });

}

export function AddPathData(app_id: number, page_path: string) {
    console.log("Add new page path to Navigation Bar ");
}

export function IsExistPage(app_id: number, page_path: string): boolean {
    axios.post("/check", {
        app_id: app_id,
        page_path: page_path,
    })
        .then(function (response) {
            return response.data;
        })
        .catch(function (response) {
            console.log(response);
            return true;
        });
    return true;
}