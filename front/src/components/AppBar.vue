<template>
  <v-app-bar app color="primary" dark clipped-left>
    <div class="d-flex align-center">
      <!-- doci icon <v-img
          alt="Vuetify Logo"
          class="shrink mr-2"
          contain
          src="https://cdn.vuetifyjs.com/images/logos/vuetify-logo-dark.png"
          transition="scale-transition"
          width="40"
        /> -->

      Doci
    </div>

    <v-spacer></v-spacer>
    <v-btn icon>
      <v-icon>mdi-pen-plus</v-icon>
      New
    </v-btn>
    <v-btn href="" target="_blank" icon>
      <v-icon>mdi-github</v-icon>
    </v-btn>
    <v-btn href="" target="_blank" text>
      <span class="mr-2">Login</span>
      <v-icon>mdi-open-in-new</v-icon>
    </v-btn>
  </v-app-bar>
</template>
<script lang="ts">
import { Vue, Component } from "vue-property-decorator";

@Component
export default class AppBar extends Vue {}
</script>
