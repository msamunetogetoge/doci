<template>
  <user-page />
</template>

<script lang="ts">
import Vue from "vue";
import UserPage from "../components/UserPage.vue";

export default Vue.extend({
  name: "SignUp",
  // todo: $route.params などでuser_id, app_id をedit-page に渡す。
  components: {
    UserPage,
  },
});
</script>