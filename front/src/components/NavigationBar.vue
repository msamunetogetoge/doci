<template>
  <v-card>
    <v-navigation-drawer
      app
      clipped
      v-model="drawer"
      :mini-variant.sync="mini"
      permanent
    >
      <v-list-item class="px-2">
        <v-list-item-avatar>
          <v-icon>mdi-account-box</v-icon>
        </v-list-item-avatar>

        <v-list-item-title>UserName</v-list-item-title>

        <v-btn icon @click.stop="mini = !mini">
          <v-icon>mdi-chevron-left</v-icon>
        </v-btn>
      </v-list-item>

      <v-divider></v-divider>

      <v-list dense>
        <v-list-item v-for="item in items" :key="item.title" link>
          <v-list-item-icon>
            <v-icon>{{ item.icon }}</v-icon>
          </v-list-item-icon>

          <v-list-item-content>
            <v-list-item-title>{{ item.title }}</v-list-item-title>
          </v-list-item-content>
        </v-list-item>
      </v-list>
      <template>
        <v-treeview
          :items="items_folder"
          :open-on-click="true"
          @click="mini = !mini"
          :transition="true"
        >
          <template v-slot:prepend="{ item }">
            <v-icon
              v-if="!item.parent"
              v-text="
                `mdi-${item.id === 1 ? 'file-document-multiple ' : 'folder'}`
              "
            ></v-icon>
          </template>
        </v-treeview>
      </template>
    </v-navigation-drawer>
  </v-card>
</template>
<script lang="ts">
import { Vue, Component } from "vue-property-decorator";

@Component
export default class NavBar extends Vue {
  drawer = true;
  items = [
    { title: "Home", icon: "mdi-home-city" },
    { title: "My Account", icon: "mdi-account" },
    { title: "Users", icon: "mdi-account-group-outline" },
  ];
  mini = true;
  initiallyOpen = ["public"];
  files = {
    html: "mdi-language-html5",
    js: "mdi-nodejs",
    json: "mdi-code-json",
    md: "mdi-language-markdown",
    pdf: "mdi-file-pdf",
    png: "mdi-file-image",
    txt: "mdi-file-document-outline",
    xls: "mdi-file-excel",
  };
  tree = [];
  items_folder = [
    {
      id: 1,
      name: "Documents",
      children: [
        {
          id: 2,
          name: "Core team",
          children: [
            {
              id: 201,
              name: "John",
            },
            {
              id: 202,
              name: "Kael",
            },
            {
              id: 203,
              name: "Nekosaur",
            },
            {
              id: 204,
              name: "Jacek",
            },
            {
              id: 205,
              name: "Andrew",
            },
          ],
        },
        {
          id: 3,
          name: "Administrators",
          children: [
            {
              id: 301,
              name: "Mike",
            },
            {
              id: 302,
              name: "Hunt",
            },
          ],
        },
        {
          id: 4,
          name: "Contributors",
          children: [
            {
              id: 401,
              name: "Phlow",
            },
            {
              id: 402,
              name: "Brandon",
            },
            {
              id: 403,
              name: "Sean",
            },
          ],
        },
      ],
    },
  ];
  open = [1, 2];
}
</script>

