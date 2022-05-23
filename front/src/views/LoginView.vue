<template>
  <login-form />
</template>

<script lang="ts">
import Vue from "vue";
import LoginForm from "../components/LoginForm.vue";

export default Vue.extend({
  name: "Login",
  // todo: $route.params などでuser_id, app_id をedit-page に渡す。
  components: {
    LoginForm,
  },
});
</script>
