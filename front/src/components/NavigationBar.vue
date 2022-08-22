<template>
  <v-card>
    <v-navigation-drawer
      app
      clipped
      v-model="drawer"
      :mini-variant.sync="mini"
      permanent
      width="512"
    >
      <v-list-item class="px-2">
        <v-list-item-avatar>
          <v-icon>mdi-account-box</v-icon>
        </v-list-item-avatar>

        <v-list-item-title>{{ user_name }}</v-list-item-title>

        <v-btn icon @click.stop="mini = !mini">
          <v-icon>mdi-chevron-left</v-icon>
        </v-btn>
      </v-list-item>

      <v-divider></v-divider>
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
          <!-- hover が上手く動いてない -->
          <template v-slot:label="{ item }">
            <v-hover v-slot:default="{ hover }">
              <div>
                <span> {{ item.name }} </span>
                <span v-if="hover">{{ item.name }}</span>
              </div>
            </v-hover>
          </template>
          <template v-slot:prepend="{ item }">
            <v-icon v-text="GetIcon(item)"></v-icon>
          </template>
          <template v-slot:append="{ item }">
            <v-btn
              icon
              v-if="item.depth != HIERARCHY_TOP_NUMBER"
              @click="DeleteItems(item.id)"
            >
              <v-icon v-text="files.delete"></v-icon>
            </v-btn>
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
import { DeletePages, GetPage } from "../utils/page-util";
import { Vue, Component, Prop } from "vue-property-decorator";
import { GetFolders, Hierarchy } from "../utils/hierarchy-utils";

// const HIERARCHY_TOP_NUMBER = 0;
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

  // db のpublic.page_hierarchy でparent_path= app_name. child_path=app_name のdepthを表す定数。
  // back 側にも同じ定数があるので合わせる必要がある。
  readonly HIERARCHY_TOP_NUMBER = 0;

  dialog = false;
  drawer = true;
  mini = true;
  show_tree = true;
  // treeviewで、最初に開いておくフォルダなど
  open = [];

  // ユーザー名
  user_name = this.$store.state.user_name;

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

  items_folder: Hierarchy[] = []; // tree-view の中身

  // mount が終わったらプロパティの初期値をtree-view にセットする
  mounted() {
    this.items_folder = [
      {
        app_id: this.AppId,
        name: this.AppName,
        depth: this.HIERARCHY_TOP_NUMBER,
        id: undefined,
        children: [],
      },
    ];
  }

  // tree-viewのボタンをクリックしたときに呼ばれる
  // 確認したらフォルダ(を含む)以下を削除する
  async DeleteItems(hierarchy_id: number): Promise<void> {
    let success = await DeletePages(hierarchy_id);
    if (success) {
      alert("削除しました");
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
      console.log(page);
      this.$emit("StartEdit", page.page_path, page.md);
    }
  }

  // tree-view のアイコンを決める関数
  GetIcon(item: Hierarchy): string {
    if (item.depth === this.HIERARCHY_TOP_NUMBER) {
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

