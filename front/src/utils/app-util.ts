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

export async function get_created_app_doc(userid: number): Promise<appinfo[]> {
    const url = "/create_doc/" + userid.toString();
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