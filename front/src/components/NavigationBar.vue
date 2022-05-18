<template>
  <v-card>
    <v-navigation-drawer
      app
      clipped
      v-model="drawer"
      :mini-variant.sync="mini"
      permanent
    >
      <v-list-item class="px-2">
        <v-list-item-avatar>
          <v-icon>mdi-account-box</v-icon>
        </v-list-item-avatar>

        <v-list-item-title>UserName</v-list-item-title>

        <v-btn icon @click.stop="mini = !mini">
          <v-icon>mdi-chevron-left</v-icon>
        </v-btn>
      </v-list-item>

      <v-divider></v-divider>

      <v-list dense>
        <v-list-item v-for="item in items" :key="item.title" link>
          <v-list-item-icon>
            <v-icon>{{ item.icon }}</v-icon>
          </v-list-item-icon>

          <v-list-item-content>
            <v-list-item-title>{{ item.title }}</v-list-item-title>
          </v-list-item-content>
        </v-list-item>
      </v-list>
      <template>
        <v-treeview
          v-if="show_tree"
          :items="items_folder"
          :dense="true"
          :open="open"
          @click="mini = !mini"
          :transition="true"
          :load-children="AddChildren"
        >
          <template v-slot:prepend="{ item }">
            <v-icon v-text="GetIcon(item)"></v-icon>
          </template>
          <template v-slot:append="{ item }">
            <!-- <v-btn icon @click="DeleteItems(item)">
              <v-icon
              v-text="files.delete"
              ></v-icon>
            </v-btn> -->
            <v-dialog v-model="dialog" max-width="600px" :retain-focus="false">
              <template v-slot:activator="{ on, attrs }">
                <v-btn icon v-if="item.depth != 1" v-bind="attrs" v-on="on">
                  <v-icon v-text="files.delete"></v-icon>
                </v-btn>
              </template>

              <v-card>
                <v-card-text> 削除しますか？ </v-card-text>
                <v-card-actions>
                  <v-spacer></v-spacer>

                  <v-btn color="blue darken-1" text @click="dialog = false">
                    No
                  </v-btn>
                  <v-btn color="blue darken-1" text @click="DeleteItems(item)">
                    Yes
                  </v-btn>
                </v-card-actions>
              </v-card>
            </v-dialog>
            <!-- <v-btn icon @click="DeleteItems(item)">
              <v-icon
              v-text="files.delete"
              ></v-icon>
              </v-btn> -->

            <v-btn icon v-if="!item.children" @click="EditItems(item)">
              <v-icon v-text="files.pencil"></v-icon>
            </v-btn>
          </template>
        </v-treeview>
      </template>
    </v-navigation-drawer>
  </v-card>
</template>
<script lang="ts">
import { DeletePages, GetPage } from "@/utils/page-util";
import { Vue, Component, Prop, Watch } from "vue-property-decorator";
import { GetFolders, Hierarchy } from "../utils/hierarchy-utils";
import EditPage from "./EditPage.vue";

@Component
export default class NavBar extends Vue {
  @Prop({ type: Number, default: 0 })
  AppId!: number;

  @Prop({ type: String, default: "app" })
  AppName!: string;

  @Prop({ type: Number, default: 0 })
  UserId!: number;

  @Prop({ default: () => [] })
  PageHierarchy!: Hierarchy[];

  dialog = false;
  drawer = true;
  mini = true;
  show_tree = true;
  // treeviewで、最初に開いておくフォルダなど
  open = [];

  //アイコン名
  files = {
    folder: "mdi-folder",
    folders: "mdi--file-document-multiple",
    html: "mdi-language-html5",
    js: "mdi-nodejs",
    json: "mdi-code-json",
    md: "mdi-language-markdown",
    pdf: "mdi-file-pdf",
    png: "mdi-file-image",
    txt: "mdi-file-document-outline",
    xls: "mdi-file-excel",
    delete: "mdi-delete",
    pencil: "mdi-lead-pencil",
  };

  items = [
    { title: "Real-Time", icon: "mdi-clock" },
    { title: "Audience", icon: "mdi-account" },
    { title: "Conversions", icon: "mdi-flag" },
  ];

  items_folder: Hierarchy[] = []; // tree-view の中身

  // mount が終わったらプロパティの初期値をtree-view にセットする
  mounted() {
    // this.items_folder=this.PageHierarchy;
    this.items_folder = [
      {
        app_id: this.AppId,
        name: this.AppName,
        depth: 1,
        id: undefined,
        children: [],
      },
    ];
  }

  // tree-viewのボタンをクリックしたときに呼ばれる
  // 確認したらフォルダ(を含む)以下を削除する
  async DeleteItems(item: Hierarchy): Promise<void> {
    let success = await DeletePages(item);
    if (success) {
      alert(item + "を削除しました");
      this.show_tree = false;
      this.$nextTick(() => (this.show_tree = true));
      this.items_folder = [
        {
          app_id: this.AppId,
          name: this.AppName,
          depth: 1,
          id: undefined,
          children: [],
        },
      ];
    } else {
      alert("削除に失敗しました");
    }
    this.dialog = false;
  }

  // tree-viewのボタンをクリックしたときに呼ばれる
  // マークダウンファイルを編集する
  async EditItems(item: Hierarchy) {
    // page_path とその内容を取得して、EditPage.vueに渡す

    if (item.id === undefined) {
      alert("編集できません");
      return;
    } else {
      let page = await GetPage(item.id);
      this.$emit("StartEdit", page.page_path, page.md);
    }
  }

  // tree-view のアイコンを決める関数
  GetIcon(item: Hierarchy): string {
    if (item.depth === 1) {
      return this.files["folders"];
    } else if (item.children) {
      return this.files["folder"];
    } else {
      return this.files["md"];
    }
  }

  // tree-view のアイコンをクリックしたら、フォルダの中身を検索して追加する関数
  async AddChildren(parent: Hierarchy) {
    let child: Hierarchy[] = await GetFolders(
      this.AppId,
      parent.depth,
      parent.name,
      parent.id
    );

    if (parent.children) {
      for (let index = 0; index < child.length; index++) {
        const element = child[index];
        parent.children.push(element);
      }
    } else {
      // 何もしない
    }
  }
}
</script>

