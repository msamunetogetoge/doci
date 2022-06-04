<template>
  <v-row dense>
    <v-card>
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
                :items-per-page="5"
                class="elevation-1"
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
                      <v-text-field label="app name" required></v-text-field>
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
                <small>*indicates required field</small>
              </v-card-text>
              <v-card-actions>
                <v-spacer></v-spacer>
                <v-btn color="blue darken-1" text @click="dialog = false">
                  Close
                </v-btn>
                <v-btn color="blue darken-1" text @click="dialog = false">
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
} from "../utils/app-util";

@Component
export default class UserPage extends Vue {
  // 参加しているdoc一覧
  joined_doc: appinfo[] = [];
  // 作成したdoc一覧
  created_doc: appinfo[] = [];

  // Create Doc dialogの表示非表示管理フラグ
  dialog = false;

  // 作成したdocの v- data-table のheader
  created_header = [
    { text: "Name", value: "name", align: "start" },
    { text: "CreatedAt", value: "createdat" },
  ];

  // 参加しているdocの v- data-table のheader
  // joined_header = [
  //   { text: "Name", value: "name", align: "start" },
  //   { text: "CreatedBy", value: "createdby" },
  //   { text: "CreatedAt", value: "createdat" },
  // ];

  user_name = "";
  user_id = 0;

  // computed() {}
  async mounted() {
    console.log("This is user-page");
    this.user_id = this.$store.state.user_id;
    this.user_name = this.$store.state.user_name;

    // 後で追加する
    // this.joined_doc = await get_joined_app_doc(this.user_id);

    this.created_doc = await get_created_app_doc(this.user_id);
  }
  async create_doc() {
    return;
  }
}
</script>