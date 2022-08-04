import axios, { AxiosResponse, AxiosError } from "axios"
import { create_url } from "./url-util"

export interface appinfo {
    app_id: number,
    app_name: string,
    created_by: string,
    created_at: Date,
    created_at_string: string | undefined
}

export function isappinfo(item: any): item is appinfo {
    return item.id !== undefined && item.appname !== undefined && item.createdby !== undefined && item.createdat !== undefined
}

// ユーザーが作成したドキュメント情報を取得する
export async function get_created_app_doc(userid: number): Promise<appinfo[]> {
    // axiosで問い合わせるapiのurl作成
    const path: string[] = ["doc", userid.toString()];
    const url = create_url(path);
    let res: appinfo[] = [];
    await axios.get(url).then(function (response: AxiosResponse<appinfo[]>) {
        res = response.data;
        // 画面で表示する為にDateをstringに変換する
        const locale =
            res.forEach(appinfo => {
                const returned_date = new Date(appinfo.created_at);

                appinfo.created_at_string = returned_date.toLocaleString();
            })
        return res;
    }).catch(function (response: AxiosError) {
        console.log(response.message);
    })
    return res;
}

export async function get_joined_app_doc(userid: number): Promise<appinfo[]> {

    // axiosで問い合わせるapiのurl作成
    const path: string[] = ["member", userid.toString()];
    const url = create_url(path);

    let res: appinfo[] = [];
    await axios.get(url).then(function (response: AxiosResponse<appinfo[]>) {
        res = response.data;
        return res;
    }).catch(function (response: AxiosError) {
        console.log(response.message);
    })
    return res
}

export async function try_create_doc(userid: number, appname: string): Promise<boolean> {

    let success = false;

    // axiosで問い合わせるapiのurl作成
    const path: string[] = ["doc"];
    const url = create_url(path);

    await axios.post(url, {
        created_by: userid,
        app_name: appname
    }).then(function () {
        success = true;
    }).catch(() => {
        success = false;
    })
    return success;
}