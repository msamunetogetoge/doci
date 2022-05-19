<template>
  <v-container style="height: 100%">
    <app-bar
      @GivePagePath="GetPagePath"
      @New="SetNew"
      :app-name="app_name"
      :app-id="app_id"
    />

    <v-card color="grey lighten-4" flat tile class="pa-md-4 mx-lg-auto">
      <v-toolbar dense>
        <v-toolbar-title v-bind:value="page_path"
          >{{ page_path }}
        </v-toolbar-title>

        <v-spacer></v-spacer>

        <v-btn @click="ClickViewButton" v-if="show_md">
          <v-icon>mdi-play</v-icon>
          View
        </v-btn>

        <v-btn @click="StartEditPage">
          <v-icon>mdi-lead-pencil</v-icon>
          Edit
        </v-btn>
      </v-toolbar>
    </v-card>

    <nav-bar
      @StartEdit="StartEdit"
      :user-id="user_id"
      :app-id="app_id"
      :app-name="app_name"
      :page-hierarchy="items_folder"
      v-if="ShowNavBar"
      style=""
    >
    </nav-bar>
    <v-row>
      <v-col v-if="show_md">
        <vue-simplemde
          v-model="markdown"
          ref="markdownEditor"
          :configs="md_configs"
        />
      </v-col>

      <v-col>
        <v-card class="overflow-y-auto" v-if="html !== ''">
          <v-card-title>Output HTML</v-card-title>
          <v-card-text class="purehtml" filled v-html="html"> </v-card-text>
        </v-card>
      </v-col>
    </v-row>
    <v-row>
      <v-spacer></v-spacer>
      <v-btn depressed @click="Convert" v-if="show_md && editing"> 変換</v-btn>
      <v-spacer></v-spacer>
      <v-btn v-if="page_path !== '/' && show_md && editing" @click="UpdatePage"
        >更新</v-btn
      >
      <v-btn v-if="page_path !== '/' && show_md && !editing" @click="AddPage"
        >作成</v-btn
      >
      <v-spacer></v-spacer>
    </v-row>
  </v-container>
</template>

<script lang="ts">
import { Vue, Component, Prop } from "vue-property-decorator";
import VueSimplemde from "vue-simplemde";
import markdownIt from "markdown-it";
import markdownItPlantuml from "markdown-it-plantuml";
import NavBar from "./NavigationBar.vue";
import AppBar from "./AppBar.vue";
import PageToolBar from "./ToolBar.vue";
import { IsExistPage, AddOrUpdate } from "../utils/page-util";
import { Hierarchy } from "../utils/hierarchy-utils";

import "../assets/style/markdown.css";
import "../assets/style/simplemde.min.css";
import "github-markdown-css";

const md = new markdownIt();
md.use(markdownItPlantuml);

@Component({
  components: {
    AppBar,
    NavBar,
    PageToolBar,
    VueSimplemde,
  },
})
export default class EditPage extends Vue {
  @Prop({ type: Number, default: 0 })
  app_id!: number;
  @Prop({ type: Number, default: 0 })
  user_id!: number;
  @Prop({ type: String, default: "app" })
  app_name!: string;

  page_path = "/";
  // 更新/作成ボタンの切り替え制御フラグ
  editing = true;
  // markdown入力部分の表示制御フラグ
  show_md = true;
  markdown = "";
  html = "";
  ShowNavBar = true;

  // simplemde のconfig
  md_configs = {
    spellChecker: false,
    toolbar: [
      "bold",
      "italic",
      "heading",
      "heading-smaller",
      "heading-bigger",
      "|",
      "code",
      "quote",
      "link",
      "|",
      "unordered-list",
      "ordered-list",
      "table",
      "horizontal-rule",
      "|",
      "guide",
    ],
  };

  items_folder: Hierarchy[] = [
    {
      app_id: this.app_id,
      name: this.app_name,
      depth: 1,
      id: undefined,
      children: [],
    },
  ];
  created() {
    // ログイン、アプリ選択時にvue router から値をもらうようになったら削除する start
    this.app_id = 0;
    this.app_name = "app";
    // end
  }

  // 画面右上のEditボタンを押したときに呼ばれる。
  // マークダウン入力画面を表示し、編集モードにする。
  StartEditPage() {
    this.editing = true;
    this.show_md = true;
  }
  // nav-bar から編集ボタンを押したときに、ページを編集モードにする。
  StartEdit(page_path: string, markdown: string) {
    this.editing = true;
    this.page_path = page_path;
    this.markdown = markdown;
    this.Convert();
  }

  ClickViewButton() {
    this.show_md = !this.show_md;
    this.editing = true;
  }

  // nav-barに表示される頁構造を更新する
  UpdatePages() {
    this.ShowNavBar = false;
    this.$nextTick(() => (this.ShowNavBar = true));
  }

  // app_id = 0;
  // page_path = "index.md";
  GetPagePath(page_path: string) {
    console.log("GetPAgePAth is Calling page_path = " + page_path);
    this.page_path = page_path;
  }

  //markdown 入力欄に入力されているデータをhtmlに変換する
  Convert() {
    this.html = md.render(this.markdown);
  }

  SetNew() {
    this.editing = false;
    this.markdown = "";
  }

  // 作成ボタン/更新ボタンを押したときに呼ばれる。
  // データを登録する。
  async AddPage() {
    if (await IsExistPage(this.app_id, this.page_path)) {
      alert("既に存在するページです");
    } else {
      try {
        // ページファイルを追加する
        await AddOrUpdate(this.app_id, this.page_path, this.markdown);
        // nav bar のページ構造部分を初期化する
        this.UpdatePages();
        this.items_folder = [];
        this.items_folder = [
          {
            app_id: this.app_id,
            name: this.app_name,
            depth: 1,
            id: undefined,
            children: [],
          },
        ];
        this.markdown = "";
        alert("データを登録しました。");
      } catch (error) {
        alert(error);
      }
    }
  }
  // 更新ボタンを押したときに呼ばれる。
  // マークダウンを更新する。
  async UpdatePage() {
    try {
      // ページファイルをセーブする
      await AddOrUpdate(this.app_id, this.page_path, this.markdown);
      // nav bar のページ構造部分を初期化する
      this.UpdatePages();
      this.items_folder = [];
      this.items_folder = [
        {
          app_id: this.app_id,
          name: this.app_name,
          depth: 1,
          id: undefined,
          children: [],
        },
      ];
      alert("データを更新しました。");
    } catch (error) {
      alert(error);
    }
  }
}
</script>
