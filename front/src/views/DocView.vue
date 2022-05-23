<template>
  <edit-page />
</template>

<script lang="ts">
import Vue from "vue";
import EditPage from "../components/EditPage.vue";

export default Vue.extend({
  name: "Doc",
  // todo: $route.params などでuser_id, app_id をedit-page に渡す。
  components: {
    EditPage,
  },
});
</script>
