<template>
  <sign-up-form />
</template>

<script lang="ts">
import Vue from "vue";
import SignUpForm from "../components/SignUpForm.vue";

export default Vue.extend({
  name: "SignUp",
  components: {
    SignUpForm,
  },
});
</script>