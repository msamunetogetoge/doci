import axios, { AxiosError, AxiosResponse } from "axios"
import { create_url } from "./url-util";


// pageの情報を貰う時に使うインターフェース
export interface Page {
    page_path: string;
    md: string;
}
// backend側にデータを保存してもらい、成功したらナビゲーションバーにパスを追加する
export async function AddOrUpdate(app_id: number, page_path: string, data: string): Promise<void> {

    // axiosで問い合わせるapiのurl作成
    const path: string[] = ["page"];
    const url = create_url(path);

    await axios.post(url, {
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

    // axiosで問い合わせるapiのurl作成
    const path: string[] = ["page", hierarchy_id.toString()];
    const url = create_url(path);

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

    // axiosで問い合わせるapiのurl作成
    const path: string[] = ["page", id.toString()];
    const url = create_url(path);

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

    // axiosで問い合わせるapiのurl作成
    const path: string[] = ["app", app_id.toString(), "page?page_path=" + page_path];
    const url = create_url(path);

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