<template>
  <v-app>
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
      <v-btn href="" target="_blank" icon>
        <v-icon>mdi-github</v-icon>
      </v-btn>
      <v-btn href="" target="_blank" text>
        <span class="mr-2">Login</span>
        <v-icon>mdi-open-in-new</v-icon>
      </v-btn>
    </v-app-bar>

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

export default Vue.extend({
  name: "App",

  data: () => ({
    //
  }),
});
</script>
