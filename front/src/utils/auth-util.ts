import axios, { AxiosResponse, AxiosError } from "axios"

export async function login(mail_address: string, pass: string): Promise<boolean> {
    let success = false;
    await axios.post("/login", {
        mailaddress: mail_address,
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
    name: string,
    password: string,
    mailaddress: string,
}

export async function signup_user(name: string, mail_address: string, pass: string): Promise<boolean> {
    let success = false;
    const user: UserInfo = {
        name: name,
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

export async function edit_user(name: string, mail_address: string, pass: string): Promise<boolean> {
    let success = false;
    const user: UserInfo = {
        name: name,
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