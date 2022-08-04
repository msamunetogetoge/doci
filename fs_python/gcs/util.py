""" standard os module. """
import os

from google.cloud import storage

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
