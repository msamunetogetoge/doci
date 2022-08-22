<template>
  <v-container fill-height>
    <v-card class="rounded-lg mx-auto" min-width="300">
      <v-card-title> Login </v-card-title>
      <v-card-text>
        <v-form ref="form" v-model="valid" lazy-validation>
          <v-text-field
            v-model="id"
            :rules="idRules"
            label="USERNAME"
            required
          ></v-text-field>
          <v-text-field
            v-model="pass"
            :rules="passRules"
            label="PASSWORD"
            required
          ></v-text-field>
          <v-alert color="red lighten-2" dark v-if="display_error">
            IDかパスワードが間違っています
          </v-alert>
          <v-btn
            :disabled="!valid"
            color="success"
            class="mr-4"
            @click="tryLogin"
          >
            Login
          </v-btn>
          <v-btn color="primary" class="mr-4" @click="signUp"> SignUp </v-btn>
          <v-btn color="secondry" class="mr-4" @click="Trial"> Trial </v-btn>
        </v-form>
      </v-card-text>
    </v-card>
  </v-container>
</template>
<script lang="ts">
import { Vue, Component } from "vue-property-decorator";
import { login, get_user } from "../utils/auth-util";

@Component
export default class LoginForm extends Vue {
  valid = true;
  display_error = false;
  id = "";
  idRules = [(v: string) => !!v || "E-MAIL is required"];
  pass = "";
  passRules = [(v: string) => !!v || "PASSWORD is required"];

  computed() {
    this.display_error = false;
  }

  displayError() {
    this.display_error = true;
  }

  signUp() {
    this.$router.push("/signup");
  }

  async tryLogin() {
    let success = await login(this.id, this.pass);
    if (success) {
      this.$store.dispatch("login_state_ok");
      this.display_error = false;
      // success ==true => storeにname, idをセットしてユーザーページに飛ばす
      let user = await get_user(this.id);
      this.$store.dispatch("set_user_id", user.userid);
      this.$store.dispatch("set_user_name", user.username);
      this.$router.push("user");
    } else {
      this.$store.dispatch("login_state_no");
      this.displayError();
    }
  }

  async Trial() {
    const id = "test";
    const pass = "test";
    let success = await login(id, pass);
    if (success) {
      this.$store.dispatch("login_state_ok");
      this.display_error = false;
      // success ==true => storeにname, idをセットしてユーザーページに飛ばす
      let user = await get_user(id);
      this.$store.dispatch("set_user_id", user.userid);
      this.$store.dispatch("set_user_name", user.username);
      this.$router.push("user");
    } else {
      this.$store.dispatch("login_state_no");
      this.displayError();
    }
  }
}
</script>