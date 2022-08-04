import unittest
import os
from dotenv import load_dotenv

from .util import GoogleCloudStrageFs

load_dotenv()


class GoogleCloudStrageFsInstatnceTest(unittest.TestCase):
    """GoogleCloudStrageFs クラスのテスト

    """

    def setUp(self):
        self._wrong_cledential_file_path = "ababa.json"
        self._right_cledential_file_path = os.environ['CLEDENTIAL_FILE_PATH']
        self._dumy_cledential_file_path = os.environ['DUMY_CLEDENTIAL_FILE_PATH']
        self._error_cledential_file_path = os.environ['ERROR_CLEDENTIAL_FILE_PATH']
        self._wrong_bucket_name = "hanaaga"
        self._right_bucket_name = os.environ['BUCKET_NAME']

    @unittest.expectedFailure
    def test_fail_create_bucket_instanse_caues_wrong_cledential_file_path(self):
        """ 変なapi key のファイルパスを入力した時、GoogleCloudStrageFs のインスタンスが作れないテスト
        """
        _: GoogleCloudStrageFs = GoogleCloudStrageFs(
            cledential_key_file_path=self._wrong_cledential_file_path,
            bucket_name=self._right_bucket_name)
        pass

    @unittest.expectedFailure
    def test_fail_create_bucket_instanse_caues_bad_cledential_file(self):
        """必要な項目がセットされていないapi keyを渡した時、GoogleCloudStrageFs のインスタンスが作れないテスト
        """
        _ = GoogleCloudStrageFs(
            cledential_key_file_path=self._error_cledential_file_path,
            bucket_name=self._right_bucket_name)
        pass

    @unittest.expectedFailure
    def test_fail_create_bucket_instanse_caues_not_authorized_cledential_file(self):
        """認証に失敗するapi keyを渡した時、GoogleCloudStrageFs のインスタンスが作れないテスト
        """
        _ = GoogleCloudStrageFs(
            cledential_key_file_path=self._dumy_cledential_file_path,
            bucket_name=self._right_bucket_name)
        pass

    @unittest.expectedFailure
    def test_fail_create_bucket_instanse_caues_wrong_bucket_name(self):
        """変なバケット名を入力した時に、GoogleCloudStrageFs のインスタンスが作れないテスト
        """
        _ = GoogleCloudStrageFs(
            cledential_key_file_path=self._right_cledential_file_path,
            bucket_name=self._wrong_bucket_name)
        pass

    def test_create_bucket_instanse(self):
        """インスタンスが作成できるかテスト
        """
        _ = GoogleCloudStrageFs(
            cledential_key_file_path=self._right_cledential_file_path,
            bucket_name=self._right_bucket_name)


class GoogleCloudStrageFsUploadTest(unittest.TestCase):
    def setUp(self):
        self._cledential_file_path = os.environ['CLEDENTIAL_FILE_PATH']
        self._bucket_name = os.environ['BUCKET_NAME']
        self._file_name_test = "test_file.md"
        self._content_test = """
            # test content
            ## hoge
            """
        self.client = GoogleCloudStrageFs(
            cledential_key_file_path=self._cledential_file_path,
            bucket_name=self._bucket_name
        )

    def tearDown(self):
        try:
            self.client.delete_file(file_name=self._file_name_test)
        except Exception:
            pass

    def test_upload(self):
        """ファイルをアップロード出来るかのテスト
        """
        self.client.upload_file_content(
            file_name=self._file_name_test, content=self._content_test)


class GoogleCloudStrageFsDownloadTest(unittest.TestCase):
    def setUp(self):
        self._cledential_file_path = os.environ['CLEDENTIAL_FILE_PATH']
        self._bucket_name = os.environ['BUCKET_NAME']
        self._file_name_test = "test_file.md"
        self._content_test = """
            # test content
            ## hoge
            """
        self.client = GoogleCloudStrageFs(
            cledential_key_file_path=self._cledential_file_path,
            bucket_name=self._bucket_name
        )
        self.client.upload_file_content(
            file_name=self._file_name_test, content=self._content_test)

    def tearDown(self):
        try:
            self.client.delete_file(file_name=self._file_name_test)
        except Exception:
            pass

    def test_get_file_content(self):
        """ファイルをダウンロード出来るかのテスト
        """
        content = self.client.get_file_content(file_name=self._file_name_test)
        self.assertEqual(content, self._content_test)

    @unittest.expectedFailure
    def test_fail_get_filecontent(self):
        """存在しないファイルの中身を読み込もうとした時にエラーが出るテスト
        """
        content = self.client.get_file_content(file_name="havnwaiogew.hae")

    class GoogleCloudStrageFsUpdateTest(unittest.TestCase):

        def setUp(self):
            self._cledential_file_path = os.environ['CLEDENTIAL_FILE_PATH']
            self._bucket_name = os.environ['BUCKET_NAME']
            self._file_name_test = "test_file.md"
            self._content_test = """
                # test content
                ## hoge
                """
            self._content_update_test = """
                # test content
                ## hoge
                ### hage
                """
            self.client = GoogleCloudStrageFs(
                cledential_key_file_path=self._cledential_file_path,
                bucket_name=self._bucket_name
            )
            self.client.upload_file_content(
                file_name=self._file_name_test, content=self._content_test)

        def tearDown(self):
            try:
                self.client.delete_file(file_name=self._file_name_test)
            except Exception("ファイルのアップロードに失敗していた") as e:
                print(e)
                pass

        def test_update_file_content(self):
            """ファイルを更新出来るかのテスト
            """
            self.client.update_file_content(
                file_name=self._file_name_test, content=self._content_update_test)
            content = self.client.get_file_content(
                file_name=self._file_name_test)
            self.assertEqual(content, self._content_update_test)

        @unittest.expectedFailure
        def test_update_not_exist_file_content(self):
            """存在しないファイルを更新した時に失敗するテスト
            """
            self.client.update_file_content(
                file_name="aklfjsdg;:af.aegh", content=self._content_update_test)
            content = self.client.get_file_content(
                file_name=self._file_name_test)
            self.assertEqual(content, self._content_update_test)


class GoogleCloudStrageFsDeleteTest(unittest.TestCase):

    def setUp(self):
        self._cledential_file_path = os.environ['CLEDENTIAL_FILE_PATH']
        self._bucket_name = os.environ['BUCKET_NAME']
        self._file_name_test = "test_file.md"
        self._content_test = """
                # test content
                ## hoge
                """
        self.client = GoogleCloudStrageFs(
            cledential_key_file_path=self._cledential_file_path,
            bucket_name=self._bucket_name
        )
        self.client.upload_file_content(
            file_name=self._file_name_test, content=self._content_test)

    def tearDown(self):
        try:
            self.client.delete_file(file_name=self._file_name_test)
        except Exception:
            pass

    def test_delete_file_content(self):
        """ファイルを削除出来るかのテスト
        """
        self.client.delete_file(
            file_name=self._file_name_test)

    @unittest.expectedFailure
    def test_delete_not_exist_file_content(self):
        """存在しないファイルを更新した時に失敗するテスト
        """
        self.client.delete_file(
            file_name="aklfjsdg;:af.aegh")
