<template>
  <edit-page />
</template>

<script lang="ts">
import Vue from "vue";
import EditPage from "../components/EditPage.vue";

export default Vue.extend({
  name: "Home",

  components: {
    EditPage,
  },
});
</script>
