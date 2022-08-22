<template>
  <v-app-bar app color="primary" dark clipped-left>
    <div class="d-flex align-center">Doci</div>

    <v-spacer></v-spacer>
    <v-dialog v-model="dialog" persistent max-width="600px">
      <template v-slot:activator="{ on, attrs }">
        <!-- <v-btn icon @click="IsInput = true" v-bind="attrs" v-on="on"> -->
        <v-btn icon @click="InitDialog" v-bind="attrs" v-on="on">
          CREATE
        </v-btn>
      </template>
      <v-card>
        <v-card-title>
          <span class="text-h5">Input Page Path</span>
        </v-card-title>
        <v-card-text>
          <v-container>
            <v-row>
              <v-col cols="12">
                <v-text-field
                  label="ページパス"
                  required
                  v-model="page_path"
                  :rules="[rules.fobidden_char]"
                ></v-text-field>
              </v-col>
            </v-row>
          </v-container>
          <small>hoge/hage など</small>
        </v-card-text>
        <v-card-actions>
          <v-spacer></v-spacer>
          <v-btn color="blue darken-1" text @click="dialog = false">
            Close
          </v-btn>
          <v-btn
            :disabled="text_field_validation_failed"
            color="blue darken-1"
            text
            @click="SetPagePath"
          >
            Ok
          </v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>
    <v-btn href="" target="_blank" icon>
      <v-icon>mdi-github</v-icon>
    </v-btn>
  </v-app-bar>
</template>
<script lang="ts">
import { Vue, Component, Prop, Watch } from "vue-property-decorator";
import { IsExistPage } from "../utils/page-util";

@Component
export default class AppBar extends Vue {
  dialog = false;
  @Prop({ type: Boolean, default: false })
  IsInput?: boolean;

  @Prop({ type: String, default: "/" })
  PagePath!: string;

  @Prop({ type: Number, default: 0 })
  AppId!: number;

  // ダイアログのtext_fieldのリアルタイムバリデーション
  @Watch("page_path", { deep: true })
  page_pathChange(val: string, oldVal: string) {
    const regex = /[^a-zA-Z0-9/_-]/;
    if (regex.test(val)) {
      this.text_field_validation_failed = true;
    } else {
      this.text_field_validation_failed = false;
    }
  }
  // mounted のタイミングで取得する
  app_name = "";
  app_id = 0;

  // validation rule
  rules = {
    fobidden_char: (value: string) => {
      const regex = /[^a-zA-Z0-9/_-]/;
      return (
        !regex.test(value) || "使用できる文字は、半角英数字,'/', '-', '_'です。"
      );
    },
  };

  // text_fields のvalidationの成否
  // validation 失敗 => true(ダイアログのOkボタンのdisabled属性の値)
  text_field_validation_failed = true;

  // New ボタンのdialogで表示する変数
  page_path = "";
  // データ作成に使う変数
  page_path_post = "";
  mounted() {
    this.app_name = this.$store.state.app_name;
    this.app_id = this.$store.state.app_id;
  }
  // dbに登録する用のpathを作成し、既に存在するページか調べる
  async SetPagePath() {
    const regx = new RegExp("/+", "g");
    this.page_path_post = this.app_name + "/" + this.page_path;
    this.page_path_post = this.page_path_post.replaceAll(regx, "/");
    this.page_path_post = this.page_path_post + ".md";
    await this.CheckPagePath();
  }

  InitDialog() {
    this.IsInput = true;
    this.page_path = "";
  }

  // 既に存在するページのパスか調べる。
  async CheckPagePath() {
    let is_exist = await IsExistPage(this.app_id, this.page_path_post);
    if (is_exist) {
      alert(this.page_path + " はすでに存在します");
    } else {
      this.$emit("GivePagePath", this.page_path_post);
      this.$emit("New");
      this.dialog = false;
    }
  }
}
</script>
