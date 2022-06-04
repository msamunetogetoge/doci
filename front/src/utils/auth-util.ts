import axios, { AxiosResponse, AxiosError } from "axios"

export async function login(username: string, pass: string): Promise<boolean> {
    let success = false;
    await axios.post("/login", {
        username: username,
        password: pass,
    })
        .then(function (response: AxiosResponse<boolean>) {
            success = response.data
        })
        .catch(function (e: AxiosError) {
            console.log(e.message);
        });

    return success;
}

export interface UserInfo {
    user_id?: number;
    username: string;
    password?: string;
    mailaddress?: string;
}

export function isUserInfo(item: any): item is UserInfo {
    return item.username !== undefined;
}


// ユーザー情報を新規登録する。
// もしも同じusernameが使われていたらfalseを返す
export async function signup_user(username: string, mail_address: string, pass: string): Promise<boolean> {
    let success = false;
    const user: UserInfo = {
        username: username,
        password: pass,
        mailaddress: mail_address
    };
    await axios.post("/user",
        user
    )
        .then(() => {
            success = true;
        })
        .catch(function (e: AxiosError) {
            console.log(e.message);
        });

    return success;
}

// ユーザー情報を編集する。
// 失敗したらfalseを返す
export async function edit_user(username: string, mail_address: string, pass: string): Promise<boolean> {
    let success = false;
    const user: UserInfo = {
        username: username,
        password: pass,
        mailaddress: mail_address
    };
    await axios.put("/user",
        user
    )
        .then(() => {
            success = true;
        })
        .catch(function (e: AxiosError) {
            console.log(e.message);
        });

    return success;
}

// userusernameからデータを取得する
// 失敗した時は、usernameのみを返す
export async function get_user(username: string): Promise<UserInfo> {
    let res: UserInfo = {
        username: username,
    };
    const url = "user" + "/" + username;
    await axios.get(url).then(function (response: AxiosResponse<UserInfo>) {
        res = response.data;

    }).catch(function (response: AxiosError) {
        console.log(response.message);
    })
    return res;
}