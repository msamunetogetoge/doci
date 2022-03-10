<template>
  <v-app>
    <v-app-bar color="primary" dark>
      <v-toolbar-title> Doci </v-toolbar-title>
      <v-spacer></v-spacer>

      <v-btn icon>
        <v-icon>mdi-magnify</v-icon>
      </v-btn>

      <v-btn icon>
        <v-icon>mdi-heart</v-icon>
      </v-btn>
    </v-app-bar>
    <!-- <v-navigation-drawer app>Navigation Lists</v-navigation-drawer> -->
    <v-footer padless fixed>
      <v-card width="100%" class="red lighten-1 text-center">
        <v-col class="text-center" cols="12">
          {{ new Date().getFullYear() }} — <strong>Vuetify</strong>
        </v-col>
      </v-card>
    </v-footer>
    <v-main>
      <router-view />
    </v-main>
  </v-app>
</template>

<script lang="ts">
import { defineComponent } from "vue";

export default defineComponent({
  name: "App",

  data() {
    return {
      //
    };
  },
});
</script>
