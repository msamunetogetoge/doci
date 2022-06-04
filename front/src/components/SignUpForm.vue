<template>
  <v-container fill-height>
    <v-card class="rounded-lg mx-auto" min-width="400" v-if="!success">
      <v-card-title> SignUp </v-card-title>
      <v-card-text>
        <v-form ref="signup_form" v-model="valid" lazy-validation>
          <v-text-field
            v-model="name"
            :rules="nameRules"
            label="USER NAME*"
            prepend-icon="mdi-account"
            required
          ></v-text-field>
          <v-text-field
            v-model="mail"
            :rules="mailRules"
            label="E-MAIL"
            prepend-icon="mdi-email"
            required
          ></v-text-field>
          <v-text-field
            v-model="pass"
            :rules="passRules"
            label="PASSWORD*"
            prepend-icon="mdi-lock"
            required
            :append-icon="show_password ? 'mdi-eye' : 'mdi-eye-off'"
            @click:append="show_password = !show_password"
            :type="show_password ? 'text' : 'password'"
          ></v-text-field>
          <v-text-field
            v-model="pass_again"
            :rules="pass_againRules"
            label="PASSWORD AGAIN*"
            prepend-icon="mdi-lock"
            :append-icon="show_password_again ? 'mdi-eye' : 'mdi-eye-off'"
            required
            @click:append="show_password_again = !show_password_again"
            :type="show_password_again ? 'text' : 'password'"
          ></v-text-field>
        </v-form>
        <small>*indicates required field</small>
      </v-card-text>
      <v-card-actions>
        <v-btn
          :disabled="!valid"
          color="success"
          class="mr-4"
          @click="valid_signup"
        >
          SignUp
        </v-btn>
      </v-card-actions>
    </v-card>
  </v-container>
</template>
<script lang="ts">
import { Vue, Component, Prop } from "vue-property-decorator";
import { signup_user, edit_user, get_user } from "../utils/auth-util";

@Component
export default class SignUpForm extends Vue {
  // SignUpボタンが使えるかのフラグ
  valid = false;
  mail = "";
  mailRules = [(v: string) => /.+@.+\..+/.test(v) || "Invalid E-MAIL."];
  pass = "";
  passRules = [(v: string) => !!v || "PASSWORD is required"];
  name = "";
  nameRules = [(v: string) => !!v || "NAME is required"];
  pass_again = "";
  pass_againRules = [(v: string) => v !== this.pass || "Don't match PASSWORD"];

  //password を見せるかのフラグ
  show_password = false;
  show_password_again = false;

  success = false;

  @Prop({ type: String, default: "" })
  given_name!: string;
  @Prop({ type: String, default: "" })
  given_mail!: string;

  // computed() {}
  mounted() {
    this.name = this.given_name;
    this.mail = this.given_mail;
  }

  // formのバリデーションに合格したらユーザー登録を行う
  async valid_signup() {
    await (this.$refs.signup_form as any).validate();
    if (this.valid) {
      await this.trySignUp();
    } else {
      alert("不適切な入力のフィールドがあります。");
    }
  }

  // SignUpが成功したらユーザーページに飛ばす
  async trySignUp() {
    if (this.given_name === "") {
      this.success = await signup_user(this.name, this.mail, this.pass);
    } else {
      this.success = await edit_user(this.name, this.mail, this.pass);
    }
    // success ==true => storeにname, idをセットしてユーザーページに飛ばす
    if (this.success) {
      const user = await get_user(this.name);
      this.$store.dispatch("login_state_ok");
      this.$store.dispatch("set_user_id", user.user_id);
      this.$store.dispatch("set_user_name", user.username);
      this.$router.push("/user");
    } else {
      alert("登録に失敗しました");
    }
  }
}
</script>