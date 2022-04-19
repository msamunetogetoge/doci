import axios, {AxiosResponse } from "axios"

// backend側にデータを保存してもらい、成功したらナビゲーションバーにパスを追加する
export async function Save(app_id: number, page_path: string, data: string): Promise<void> {
    await axios.post("/add", {
        app_id: app_id,
        page_path: page_path,
        page_data: data,
    })
        .then(function (response) {
            // 何もしない
        })
        .catch(function (response) {
            alert(response);
        });

}

// まだ使うかわからない
// export function AddPathData(app_id: number, page_path: string) {
//     console.log("Add new page path to Navigation Bar ");
// }

// dbに問い合わせて、使いたいパスがすでに存在するか確かめる。
// 存在する -> true
export async function IsExistPage(app_id: number, page_path: string): Promise<boolean>{
    let is_exist = false;
    await axios.post("/check", {
        app_id: app_id,
        page_path: page_path,
    })
        .then(function (response: AxiosResponse<boolean>) {
            is_exist = response.data;
        })
        .catch(function (response) {
            is_exist = true;
        }).finally(()=>{
            return is_exist;
        });
    return is_exist;

}