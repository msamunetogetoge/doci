// urlを作成するクラス
// 開発時かbuild時かで、apiのURLを使い分ける。
// 開発時のURLはvue,config.jsに記載されている
export class UrlBuilder {
    // production build か判定するフラグ
    private isProduction: boolean;
    // 作成されたURLを格納する変数
    private url: string;

    // productionか否かフラグをセットし、urlの初期値を決める
    constructor() {
        console.log(process.env);
        if (process.env.VUE_APP_PROXY_DOMAIN != undefined) {
            this.isProduction = true;
        }
        else {
            this.isProduction = false;
        }
        if (this.isProduction) {
            this.url = process.env.VUE_APP_PROXY_DOMAIN;
        }
        else {
            this.url = "";
        }

    }
    // url を組み立てる
    add(path: string[]) {
        for (const index in path) {
            this.url += "/";
            this.url += path[index];
        }

    }
    // 作成したurlを返す
    get_url(): string {
        return this.url;
    }

}

// UrlBuilder でurlを作成する関数
export function create_url(paths: string[]): string {
    const urlBuilder = new UrlBuilder();
    urlBuilder.add(paths);
    const url = urlBuilder.get_url();
    return url;
}