import axios, { AxiosError, AxiosResponse } from "axios"
import { Hierarchy } from "./hierarchy-utils";


// pageの情報を貰う時に使うインターフェース
export interface Page {
    page_path: string;
    md: string;
}
// backend側にデータを保存してもらい、成功したらナビゲーションバーにパスを追加する
export async function AddOrUpdate(app_id: number, page_path: string, data: string): Promise<void> {
    await axios.post("/page", {
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

// dbからデータを削除する
// 成功->true ,失敗->false
export async function DeletePages(hierarchy_id: number): Promise<boolean> {
    let success = false;

    const url = "/page" + "/" + hierarchy_id;
    await axios.delete(url,
    ).then(() => {
        success = true;
    }).catch(() => {
        success = false;
    }).finally(() => {
        return success;
    })

    return success;
}

// page_hierarchyのidからpage_pathとmdの内容を取得する
export async function GetPage(id: number): Promise<Page> {
    let res: Page = { page_path: "", md: "" };
    const url = "/page" + "/" + id;
    await axios.get(url,
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
    const url = "/app" + "/" + app_id + "/page" + "?" + "page_path=" + page_path;
    console.log("In IsExistPage, url = " + url);
    await axios.get(url)
        .then(function () {
            is_exist = true;
        })
        .catch(function (response: AxiosError) {
            console.error(response.code + response.message);

            is_exist = false;
        }).finally(() => {
            return is_exist;
        });
    return is_exist;

}