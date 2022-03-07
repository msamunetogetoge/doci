<template>
  <v-container>
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
        <v-textarea
          name="input-7-1"
          filled
          label="Input Markdown"
          auto-grow
          v-model="markdown"
        ></v-textarea>
      </v-col>
      <v-col>
        <label>Output HTML</label>
        <div v-html="html"></div>
      </v-col>
      <v-btn depressed @click="Convert"> 変換</v-btn>
    </v-row>
  </v-container>
</template>

<script lang='ts'>
import { Options, Vue } from "vue-class-component";
import markdownIt from "markdown-it";
import markdownItPlantuml from "markdown-it-plantuml";

const md = new markdownIt();
md.use(markdownItPlantuml);

// @Options({
//   props: {
//     msg: String,
//   },
// })

export default class HelloWorld extends Vue {
  markdown = "";
  html = "";

  Convert() {
    this.html = md.render(this.markdown);
  }
  // mounted() {
  //   this.msg += " Vuetify 3 Alpha";
  // }
}
</script>
