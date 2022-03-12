<template>
  <v-container>
    <page-tool-bar :editing="editing" />
    <!-- <v-navigation-drawer app clipped>Navigation Lists</v-navigation-drawer> -->
    <nav-bar> </nav-bar>
    <v-row class="text-center">
      <v-col cols="12">
        <v-img
          :src="require('../assets/logo.svg')"
          class="my-3"
          contain
          height="200"
        />
      </v-col>

      <v-col>
        <v-card>
          <v-card-title> Input Markdown </v-card-title>
          <v-card-text>
            <v-textarea
              name="input-7-1"
              filled
              auto-grow
              v-model="markdown"
              height="100%"
            ></v-textarea>
          </v-card-text>
        </v-card>
      </v-col>
      <v-btn depressed @click="Convert"> 変換</v-btn>
      <v-col>
        <v-card color="gray" height="100%">
          <v-card-title>Output HTML</v-card-title>
          <v-card-text v-html="html" filled> </v-card-text>
        </v-card>
      </v-col>
    </v-row>
    <v-row>
      <v-spacer></v-spacer>
      <v-btn v-if="editing">更新</v-btn>
      <v-btn v-if="!editing" @click="AddPage">作成</v-btn>
      <v-spacer></v-spacer>
    </v-row>
  </v-container>
</template>

<script lang="ts">
import { Vue, Component } from "vue-property-decorator";
import markdownIt from "markdown-it";
import markdownItPlantuml from "markdown-it-plantuml";
import NavBar from "./NavigationBar.vue";
import PageToolBar from "./ToolBar.vue";
import { IsExistPage, Save } from "../utils/page-util";

const md = new markdownIt();
md.use(markdownItPlantuml);

// @Options({
//   props: {
//     msg: String,
//   },
// })

@Component({
  components: {
    NavBar,
    PageToolBar,
  },
})
export default class EditPage extends Vue {
  editing = false;
  markdown = "";
  html = "";

  app_id = 0;
  page_path = "index.md";
  Convert() {
    this.html = md.render(this.markdown);
  }

  CheckSave(app_id: number, page_path: string): boolean {
    return IsExistPage(app_id, page_path);
  }

  GetFilePath(app_id: number, page_path: string): string {
    return page_path;
  }

  AddPage() {
    if (IsExistPage(this.app_id, this.page_path)) {
      alert("既に存在するページです");
    } else {
      try {
        Save(this.app_id, this.page_path, this.markdown);
      } catch (error) {
        alert(error);
      }
    }
  }
}
</script>
