import axios, { AxiosResponse, AxiosError } from "axios"

export interface appinfo {
    id: number,
    appname: string,
    createdby: string,
    createdat: Date,
}

export function isappinfo(item: any): item is appinfo {
    return item.id !== undefined && item.appname !== undefined && item.createdby !== undefined && item.createdat !== undefined
}

// ユーザーが作成したドキュメント情報を取得する
export async function get_created_app_doc(userid: number): Promise<appinfo[]> {
    const url = "/doc/" + userid.toString();
    let res: appinfo[] = [];
    await axios.get(url).then(function (response: AxiosResponse<appinfo[]>) {
        res = response.data;
        return res;
    }).catch(function (response: AxiosError) {
        console.log(response.message);
    })
    return res;
}

export async function get_joined_app_doc(userid: number): Promise<appinfo[]> {
    const url = "/member/" + userid.toString();
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
    await axios.post("/doc", {
        user_id: userid,
        app_name: appname
    }).then(function () {
        success = true;
    }).catch(() => {
        success = false;
    })
    return success;
}