import axios, { AxiosResponse } from "axios"
import { Hierarchy } from "./hierarchy-utils";


// pageの情報を貰う時に使うインターフェース
export interface Page {
    page_path: string;
    md: string;
}
// backend側にデータを保存してもらい、成功したらナビゲーションバーにパスを追加する
export async function AddOrUpdate(app_id: number, page_path: string, data: string): Promise<void> {
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

// dbからデータを削除する
// 成功->true ,失敗->false
export async function DeletePages(data: Hierarchy): Promise<boolean> {
    let success = false;
    console.log("In DeletePages data id = ");
    console.log(data.id);
    if (data.id === undefined) {
        return success;
    }
    else {
        await axios.post("/delete", {
            id: data.id
        },
        ).then(() => {
            success = true;
        }).catch(() => {
            success = false;
        }).finally(() => {
            return success;
        })
    }
    return success;
}

// page_hierarchyのidからpage_pathとmdの内容を取得する
export async function GetPage(id: number): Promise<Page> {
    let res: Page = { page_path: "", md: "" };
    await axios.post("/edit", {
        id: id
    },
    ).then((response: AxiosResponse<Page>) => {
        res = response.data
        return response.data
    }).catch((err) => {
        alert(err);
    })
    return res
}



// dbに問い合わせて、使いたいパスがすでに存在するか確かめる。
// 存在する -> true
export async function IsExistPage(app_id: number, page_path: string): Promise<boolean> {
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
        }).finally(() => {
            return is_exist;
        });
    return is_exist;

}