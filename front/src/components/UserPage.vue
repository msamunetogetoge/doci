<template>
  <v-row dense justify="center" align-content="center">
    <v-card width="80%">
      <v-card-title> Hello {{ user_name }} </v-card-title>
      <v-card-text>
        <v-col cols="12">
          <!-- 自分がオーナーのアプリ一覧 -->
          <v-card class="rounded-lg mx-auto" min-width="400">
            <v-card-title>
              <v-icon left> mdi-owner </v-icon>
              Created Doc
            </v-card-title>
            <v-card-text>
              <v-data-table
                :headers="created_header"
                :items="created_doc"
                class="elevation-1"
                @click:row="GoDocPage"
                hide-default-footer
              >
              </v-data-table>
            </v-card-text>
          </v-card>
        </v-col>
        <v-col cols="12">
          <!-- 参加しているアプリ一覧 -->
          <!-- <v-card class="rounded-lg mx-auto" min-width="400">
            <v-card-title>
              <v-icon left> mdi-user </v-icon>
              Joined Doc
            </v-card-title>
            <v-card-text>
              <v-data-table
                :headers="joined_header"
                :items="joined_doc"
                :items-per-page="5"
                class="elevation-1"
              >
              </v-data-table>
            </v-card-text>
          </v-card> -->
        </v-col>
        <v-col cols="12">
          <!-- アプリ作成 -->
          <!-- <v-card class="rounded-lg mx-auto" min-width="400">
            <v-card-actions> -->
          <v-dialog v-model="dialog" persistent max-width="600px">
            <template v-slot:activator="{ on, attrs }">
              <v-btn color="primary" dark v-bind="attrs" v-on="on">
                Create Doc
              </v-btn>
            </template>
            <v-card>
              <v-card-title>
                <span class="text-h5">App Info</span>
              </v-card-title>
              <v-card-text>
                <v-container>
                  <v-row>
                    <v-col cols="12" sm="6" md="4">
                      <v-text-field
                        label="app name"
                        required
                        v-model="app_name"
                      ></v-text-field>
                    </v-col>
                    <!-- groupmember を追加するフォームを後で作る -->
                    <!-- <v-col cols="12" sm="6" md="4">
                          <v-text-field
                            label="Legal middle name"
                            hint="example of helper text only on focus"
                          ></v-text-field>
                        </v-col> -->
                  </v-row>
                </v-container>
              </v-card-text>
              <v-card-actions>
                <v-spacer></v-spacer>
                <v-btn color="blue darken-1" text @click="dialog = false">
                  Close
                </v-btn>
                <v-btn color="blue darken-1" text @click="tryCreateDoc">
                  Create
                </v-btn>
              </v-card-actions>
            </v-card>
          </v-dialog>
          <!-- </v-card-actions>
          </v-card> -->
        </v-col>
      </v-card-text>
    </v-card>
  </v-row>
</template>
<script lang="ts">
import { Vue, Component, Prop } from "vue-property-decorator";
import {
  get_created_app_doc,
  get_joined_app_doc,
  appinfo,
  try_create_doc,
} from "../utils/app-util";

@Component
export default class UserPage extends Vue {
  // 参加しているdoc一覧 // 複数人でドキュメント編集機能を作る時に使う
  joined_doc: appinfo[] = [];
  // 作成したdoc一覧
  created_doc: appinfo[] = [];

  // Create Doc dialogの表示非表示管理フラグ
  dialog = false;

  // v-data-table のheader
  created_header = [
    // { text: "Id", value: "app_id" },
    { text: "Name", value: "app_name", align: "start" },
    // { text: "CreatedBy", value: "created_by" }, // 複数人でドキュメント編集機能を作る時に使う
    { text: "CreatedAt", value: "created_at_string" },
  ];

  // 複数人でドキュメント編集機能を作る時に使う
  // 参加しているdocの v-data-table のheader
  // joined_header = [
  //   { text: "Name", value: "name", align: "start" },
  //   { text: "CreatedBy", value: "createdby" },
  //   { text: "CreatedAt", value: "createdat" },
  // ];

  // hello user_name と画面に表示される
  user_name = "";
  // user が作ったドキュメントを検索するのに使う
  user_id = 0;
  // dialog の中で指定される。指定された名前でdocを作成する。
  app_name = "";

  // ドキュメントのページに遷移する
  GoDocPage(data: appinfo) {
    this.$store.dispatch("set_app_id", data.app_id);
    this.$store.dispatch("set_app_name", data.app_name);
    this.$router.push("/doc");

    return;
  }
  async mounted() {
    this.user_id = this.$store.state.user_id;
    this.user_name = this.$store.state.user_name;

    // 後で追加する
    // this.joined_doc = await get_joined_app_doc(this.user_id);

    this.created_doc = await get_created_app_doc(this.user_id);
  }
  // ドキュメント作成のダイアログを初期化する
  init_dialog() {
    this.dialog = false;
    this.app_name = "";
  }
  // ドキュメントを作成し、dialogを初期化する
  async tryCreateDoc() {
    const success = await try_create_doc(this.user_id, this.app_name);
    if (success) {
      this.created_doc = await get_created_app_doc(this.user_id);
    } else {
      alert("作成に失敗しました。");
    }
    this.init_dialog();
  }
}
</script>