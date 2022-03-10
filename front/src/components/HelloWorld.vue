<template>
  <v-container>
    <v-toolbar color="cyan" dark> ページの情報 </v-toolbar>
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
      <v-btn v-if="!is_create">更新</v-btn>
      <v-btn v-if="is_create">作成</v-btn>
      <v-spacer></v-spacer>
    </v-row>
  </v-container>
</template>

<script lang="ts">
import { Vue, Component } from "vue-property-decorator";
import markdownIt from "markdown-it";
import markdownItPlantuml from "markdown-it-plantuml";
import NavBar from "../components/NavigationBar.vue";

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
  },
})
export default class HelloWorld extends Vue {
  is_create = false;
  markdown = "";
  html = "";

  Convert() {
    this.html = md.render(this.markdown);
  }
}
</script>
