import axios, {AxiosResponse, AxiosError } from "axios"

// navigation bar でデータを表示するとき、パスを問い合わせするときに使うインターフェース
export interface Hierarchy{
    id?: number;
    app_id: number;
    name: string;
    depth:number;
    children?: Hierarchy[];
}

// ドキュメントのパスを取得する。appid, parent_path(name), depth を指定して、child_path たちを取得する。
export async function  GetFolders(app_id:number, depth:number, name:string,id?:number): Promise<Hierarchy[]>{
    let hierarchy_init :Hierarchy[]=[];
    
    //検索
    await axios.post("/get_hierarchy", {
        id: id,
        app_id: app_id,
        parent_path: name,
        depth: depth,
        
    })
        .then(function (response: AxiosResponse<Hierarchy[]>) {
            hierarchy_init = response.data;
        })
        .catch(function (e: AxiosError) {
            alert(e.message);
            hierarchy_init =[];
        });
    return hierarchy_init;
}