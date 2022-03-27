<template>
  <v-card color="grey lighten-4" flat tile>
    <v-toolbar dense>
      <v-toolbar-title>ページタイトル {{page_path}}</v-toolbar-title>

      <v-spacer></v-spacer>

      <v-btn>
        <v-icon>mdi-play</v-icon>
        View
      </v-btn>

      <v-btn v-if="editing">
        <v-icon>mdi-lead-pencil</v-icon>
        Edit
      </v-btn>
    </v-toolbar>
  </v-card>
</template>
<script lang="ts">
import { Vue, Component, Prop } from "vue-property-decorator";

@Component
export default class PageToolBar extends Vue {
  markdown = "";
  html = "";
  @Prop({ type: Boolean, default: false })
  editing?: boolean;
  @Prop({type:String, default: "/"})
  page_path!: string
}
</script>