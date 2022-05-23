import axios, { AxiosResponse, AxiosError } from "axios"

export async function login(mail_address: string, pass: string): Promise<boolean> {
    let success = false;
    await axios.post("/login", {
        mailaddress: mail_address,
        password: pass,
    })
        .then(function (response: AxiosResponse<boolean>) {
            console.log("resoponse = " + response.data);
            success = response.data
        })
        .catch(function (e: AxiosError) {
            console.log(e.message);
        });

    return success;
}