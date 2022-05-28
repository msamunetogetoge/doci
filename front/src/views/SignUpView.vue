<template>
  <sign-up-form />
</template>

<script lang="ts">
import Vue from "vue";
import SignUpForm from "../components/SignUpForm.vue";

export default Vue.extend({
  name: "SignUp",
  // todo: $route.params などでuser_id, app_id をedit-page に渡す。
  components: {
    SignUpForm,
  },
});
</script>