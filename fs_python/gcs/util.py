""" standard os module. """
import os

from google.cloud import storage


# class GoogleDriveFs():
#     """Google Drive内のdoci フォルダ内で、ファイルの作成、更新、削除を行うクラス。
#     """

#     def __init__(self, file_name, content):
#         self._file_name = file_name
#         self._content = content
#         self.file_id = ""

#         # Create GoogleDrive instance with authenticated GoogleAuth instance.
#         gauth = GoogleAuth()
#         # Creates local webserver and auto handles authentication.
#         gauth.LocalWebserverAuth()
#         self._drive = GoogleDrive(gauth)

#         # ファイルをフォルダから探すときのクエリ。
#         # self._drive.ListFile(self.query).GetList() のように使う
#         self.query = {'q': ''}

#     def file_already_exists(self) -> bool:
#         """self._file_name という名前のファイルが既に存在するか調べる関数。
#         存在する => true, 存在しない=> false, 同じ名前のファイルが存在する=> raise
#         true が返ってきた時は、self._file_nameを名前に持つファイルがただ一つ存在する事が保証される。

#         Raises:
#             FileNameDuplicateException: ファイル名にダブりがある時に投げるエラー。

#         Returns:
#             bool: ファイルが存在する => true, 存在しない=> false
#         """
#         query_string = f"'{FOLDER_ID}' in parents and title='{self._file_name}' and  trashed=false"
#         # query_string = f"title='{self._file_name}'"
#         self.query.update(q=query_string)
#         file_list = self._drive.ListFile(self.query).GetList()
#         if len(file_list) == 0:
#             return False
#         if len(file_list) == 1:
#             self.file_id = file_list[0]["id"]
#             return True
#         else:
#             raise FileNameDuplicateException(self._file_name)

#     def create_file(self) -> None:
#         """ファイルを新規作成する関数。
#         ファイル名=self._file_name
#         ファイルの内容=self._content
#         ファイルのアップロードに失敗した時や、既に同名の名前のファイルがある時にエラーを排出する。

#         Raises:
#             upload_error: ファイルアップロード時のエラー
#             FileAlreadyExistsException: 同名のファイルが既に存在する時のエラー
#         """
#         if not self.file_already_exists():
#             created_file = self._drive.CreateFile(
#                 {'title': self._file_name,
#                  'mimeType': 'text/markdown',
#                  "parents": [{"kind": "drive#fileLink", "id": FOLDER_ID}]})
#             # Set content of the file from given string.
#             created_file.SetContentString(self._content)
#             try:
#                 created_file.Upload()
#             except Exception as upload_error:  # TODO: Upload()が排出するエラーを調べる
#                 print(upload_error)
#                 raise upload_error
#         else:
#             raise FileAlreadyExistsException(self._file_name)

#     def update_file(self) -> None:
#         """ファイルを更新する関数。
#         ファイル名=self._file_name
#         ファイルの内容=self._content
#         ファイルのアップロードに失敗した時や、ファイル存在しない時にエラーを排出する。

#         Raises:
#             upload_error: ファイルアップロード時のエラー。
#             FileNotFoundException: ファイルが存在しない時のエラー。
#         """
#         if self.file_already_exists():
#             query_string = f"""
#             '{FOLDER_ID}' in parents and title = '{self._file_name}' and trashed = false
#             """
#             self.query.update(q=query_string)
#             queryed_file = self._drive.ListFile(self.query).GetList()[0]
#             queryed_file.SetContentString(self._content)
#             try:
#                 queryed_file.Upload()
#             except Exception as upload_error:  # TODO: Upload()が排出するエラーを調べる
#                 print(upload_error)
#                 raise upload_error
#         else:
#             raise FileNotFoundException(self._file_name)

#     def delete_file(self) -> None:
#         """ァイルを削除する(ゴミ箱に移す、でない。)関数。
#         ファイル名 = self._file_name
#         ファイルの削除に失敗した時や、ファイル存在しない時にエラーを排出する。

#         Raises:
#             delete_error: ファイルの削除に失敗した時のエラー。
#             FileNotFoundException: ファイルが存在しない時のエラー。
#         """
#         if self.file_already_exists():
#             query_string = f"""
#             '{FOLDER_ID}' in parents and title = '{self._file_name}' and trashed = false
#             """
#             self.query.update(q=query_string)
#             queryed_file = self._drive.ListFile(self.query).GetList()[0]
#             try:
#                 queryed_file.Delete()
#             except Exception as delete_error:  # TODO: Delete()が排出するエラーを調べる
#                 print(delete_error)
#                 raise delete_error
#         else:
#             raise FileNotFoundException(self._file_name)

#     def get_file_content(self) -> str:
#         """self._file_nameのファイルの中身を返す関数。
#         ファイルが存在しない時や、中身の取得に失敗した時にエラーを返す。

#         Raises:
#             content_get_error: ファイルの中身を取得するのに失敗した時のエラー。
#             FileNotFoundException: ファイルが存在しない時のエラー。

#         Returns:
#             str: _description_
#         """
#         if self.file_already_exists():
#             query_string = f"""
#             '{FOLDER_ID}' in parents and title = '{self._file_name}' and trashed = false
#             """
#             self.query.update(q=query_string)
#             try:
#                 queryed_file = self._drive.ListFile(self.query).GetList()[0]
#                 return queryed_file.GetContentString()
#             except Exception as content_get_error:
#                 print(content_get_error)
#                 raise content_get_error
#         else:
#             raise FileNotFoundException(self._file_name)


# class GoogleDriveFsException(Exception):
#     """ Google Driveでファイルの検索をした時に起こり得るエラーの基底クラス。
#     """
#     pass


# class FileNameDuplicateException(GoogleDriveFsException):
#     """ファイル名にダブりがあり、クエリで二つ以上のファイルが取得出来た時のエラー。
#     """

#     def __str__(self) -> str:
#         return (
#             f'ファイル名にダブりがあります。ファイル名は{self.args}です。'
#         )


# class FileNotFoundException(GoogleDriveFsException):
#     """ファイルが存在しない時のエラー。
#     """

#     def __str__(self) -> str:
#         return (
#             f'ファイルが存在しません。ファイル名は{self.args}です。'
#         )


# class FileAlreadyExistsException(GoogleDriveFsException):
#     """ファイルが既に存在する時のエラー。
#     """

#     def __str__(self) -> str:
#         return (
#             f"ファイルがすでに存在します。ファイル名は{self.args}です。"
#         )


class GoogleCloudStrageFs():
    """Googke Clou Storage のオブジェクトの中身を得る、アップロードする、削除する、を行うクラス。
        .env ファイルにcledential_key_file_path, bucket_name を格納している。
    """

    def __init__(self,
                 cledential_key_file_path: str,
                 bucket_name: str,
                 decode: str = "utf-8"):
        """オブジェクトの操作に必要な変数をセットする。


        Args:
            cledential_key_file_path (str): api-key のパス
            bucket_name (str): バケット名
            decode (str, optional): ダウンロードしたオブジェクトのデコード書式 Defaults to "utf-8".


        Raises:
            e: storage インスタンスの作成失敗
            Exception: _description_
        """
        self._decode = decode

        # api_keyファイルパスが正しいかチェック
        if not os.path.isfile(cledential_key_file_path):
            raise FileExistsError()
        try:
            # 変なapi_keyファイルの構造は正しいかチェック
            self._storage_client = storage.Client.from_service_account_json(
                cledential_key_file_path)
            # 変なapi_keyファイルで認証できるか
            buckets = [b.name for b in self._storage_client.list_buckets()]
        except Exception as e:
            raise e

        if bucket_name not in buckets:
            raise Exception("bucket_nameが不正")
        self._bucket = self._storage_client.bucket(bucket_name)

    def get_file_content(self, file_name: str) -> str:
        """init で指定したbucket_name バケットから、file_nameをダウンロードして返す。
        Args:
            file_name (str): gcsでのオブジェクト名

        Raises:
            e: ダウンロードに失敗した時のエラー

        Returns:
            str: google cloud strage のオブジェクトの中身
        """
        blob = self._bucket.blob(file_name)
        # blob = bucket.blob("honyanya.txt")
        try:
            content = blob.download_as_string().decode(self._decode)
            return content
        except Exception as e:
            print(e)
            raise e

    def upload_file_content(self, file_name: str, content: str) -> None:
        """init で指定したbucket_name バケットに、
        ファイル名:self._file_name、中身: self._content
        のファイルをアップロードする。
        Args:
            file_name (str): gcsでのオブジェクト名
            content (str): オブジェクトの中身

        Raises:
            e: アップロード失敗
        """
        blob = self._bucket.blob(file_name)
        try:
            blob.upload_from_string(
                content, content_type='text/markdown')
        except Exception as e:
            print(e)
            raise e

    def update_file_content(self, content: str, file_name: str) -> None:
        """既に存在するオブジェクトのデータを更新する。
        Args:
            file_name (str): gcsでのオブジェクト名
            content (str): オブジェクトの中身
        Raises:
            e:存在しないオブジェクトを更新した時のエラー
        """
        blob = self._bucket.blob(file_name)
        try:
            _ = blob.download_as_string(start=0, end=0)
        except Exception as e:
            print(e)
            raise e
        self.upload_file_content(file_name=file_name, content=content)

    def delete_file(self, file_name: str) -> None:
        """init で指定したbucket_name バケットから、
        オブジェクト名:self._file_name
        のファイルを削除する。
        Args:
            file_name (str): gcsでのオブジェクト名

        Raises:
            e:削除失敗
        """
        blob = self._bucket.blob(file_name)
        try:
            blob.delete()
        except Exception as e:
            print(e)
            raise e
