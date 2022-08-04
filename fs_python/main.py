""" standard json module."""
import json
import traceback
import os

from flask import Flask, request, Response
from dotenv import load_dotenv

from gcs.util import GoogleCloudStrageFs


app = Flask(__name__)

app.config['JSON_AS_ASCII'] = False

# .envファイル読み込み
load_dotenv()

# api keyのファイルパス
API_KEY_PATH: str = os.environ['CLEDENTIAL_FILE_PATH']
# google storageのバケット名
BUCKET_NAME: str = os.environ['BUCKET_NAME']


# ToDo
# 1. flask で "/" にPOSTが来たら引数を持っているかチェックする
# 2. drive.util 内にgoogle drive内のファイルを操作するクラスを作成する。
# 要件は以下の通り
#   自動で認証を実行する
#   doci フォルダ(id=13GbaoTXLFP1V-CMyo-kPSSMJBnIxXl3_) 内にfile_name のmarkdown ファイルがあるか検索する
#   存在しない時はtitle = file_name, content_string= content となるデータを作成できる。
#   存在する時はcontent_string= content となるデータにupdateできる。
#   操作が成功したら、status code 200 失敗したらログを書き出し、400 や500 を返す。
# 3. status code 200 以外が返ったら処理失敗

@app.route("/", methods=['POST', 'PUT' 'DELETE'])
@app.route("/<_file_name>", methods=['GET'])
def edit_document(_file_name=None) -> Response:
    """_summary_: file_name, content をjson形式で受け取り、グーグルドライブ内でファイルの作成、更新、削除を行う。受け取るhttp method は GET,POST, PUT, DELETE

    Returns:
        Response:PUT, POST, DELETE 以外のmethodが来たらstatuscode 400を返す。
        methodの処理に成功したらstatus code 200, 失敗したらstatus code 500とメッセージを返す。
    """
    file_info: GoogleCloudStrageFs = GoogleCloudStrageFs(
        cledential_key_file_path=API_KEY_PATH, bucket_name=BUCKET_NAME)
    if request.method == 'GET':
        file_name: str = _file_name
        try:
            content = file_info.get_file_content(file_name=file_name)
            return Response(response=json.dumps({'content': content}), status=200)
        except Exception as content_get_error:
            return Response(
                response=json.dumps(
                    {'message': create_error_message(content_get_error)}),
                status=500)

    # foo = request.json['key'] で失敗すると、400 Bad Request Error がraise される

    if request.method == 'POST':
        try:
            file_name: str = request.json["file_name"]
            content: str = request.json["content"]
            file_info.upload_file_content(file_name=file_name, content=content)
            return Response(status=200)
        # except FileAlreadyExistsException as file_already_exists:
        #     return Response(response=json.dumps({'message': create_error_message(file_already_exists)}),
        #                     status=400)
        except Exception as create_error:
            return Response(response=json.dumps({'message': create_error_message(create_error)}),
                            status=500)

    elif request.method == 'PUT':
        try:
            file_name: str = request.json["file_name"]
            content: str = request.json["content"]
            file_info.update_file_content(file_name=file_name, content=content)
            return Response(status=200)
        # except FileNotFoundError as file_not_found:
        #     return Response(response=json.dumps({'message': create_error_message(file_not_found)}),
        #                     status=400)
        except Exception as update_error:
            return Response(response=json.dumps({'message': create_error_message(update_error)}),
                            status=500)
    elif request.method == 'DELETE':
        try:
            file_name: str = request.json["file_name"]
            file_info.delete_file(file_name=file_name)
            return Response(status=200)
        # except FileNotFoundException as file_not_found:
        #     return Response(response=json.dumps({'message': create_error_message(file_not_found)}),
        #                     status=400)
        except Exception as delete_error:
            return Response(response=json.dumps({'message': create_error_message(delete_error)}),
                            status=500)

    else:
        return Response(response=json.dumps({'message': 'allowed methods are POST, PUT or DELETE'}),
                        status=400)


# def set_file_information(file_name: str, content: str) -> GoogleDriveFs:
#     """Google Driveのファイルを操作するインスタンス

#     Args:
#         file_name (str): ファイル名
#         content (str): ファイルの内容

#     Returns:
#         GoogleDriveFs: GoogleDrive instance with authenticated GoogleAuth instance.
#     """
#     return GoogleDriveFs(file_name=file_name, content=content)


def create_error_message(e: Exception) -> str:
    """エラー名+エラーの最終行の文字列を返す

    Args:
        e (Exception):エラー

    Returns:
        str: エラー名+エラーの最終行
    """
    return traceback.format_exception_only(type(e), e)


if __name__ == "__main__":
    app.run(host='127.0.0.1', port=8080, debug=True)
