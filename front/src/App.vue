<template>
  <v-app>
    <app-bar />
    <v-main>
      <router-view />
    </v-main>
    <v-footer color="primary" dark app padless absolute>
      <v-card width="100%" class="lighten-1 text-center">
        <v-col class="text-center" cols="12">
          {{ new Date().getFullYear() }} — <strong>masamunenoheya</strong>
        </v-col>
      </v-card>
    </v-footer>
  </v-app>
</template>

<script lang="ts">
import Vue from "vue";
import AppBar from "./components/AppBar.vue";

export default Vue.extend({
  name: "App",

  data: () => ({
    //
  }),
  components: {
    AppBar,
  },
});
</script>
