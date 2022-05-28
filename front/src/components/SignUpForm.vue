<template>
  <v-container fill-height>
    <v-card
      class="rounded-lg mx-auto"
      color="blue-grey lighten-4"
      min-width="400"
      v-if="!success"
    >
      <v-card-title> SignUp </v-card-title>
      <v-card-text>
        <v-form ref="form" v-model="valid" lazy-validation>
          <v-text-field
            v-model="id"
            :rules="idRules"
            label="E-MAIL"
            required
          ></v-text-field>
          <v-text-field
            v-model="name"
            :rules="nameRules"
            label="NAME"
            required
          ></v-text-field>
          <v-text-field
            v-model="pass"
            :rules="passRules"
            label="PASSWORD"
            required
          ></v-text-field>
          <v-text-field
            v-model="pass_again"
            :rules="pass_againRules"
            label="PASSWORD AGAIN"
            required
          ></v-text-field>

          <v-btn
            :disabled="!valid"
            color="success"
            class="mr-4"
            @click="trySignUp"
          >
            SignUp
          </v-btn>
        </v-form>
      </v-card-text>
    </v-card>
  </v-container>
</template>
<script lang="ts">
import { Vue, Component, Prop } from "vue-property-decorator";
import { signup_user, edit_user } from "../utils/auth-util";

@Component
export default class SignUpForm extends Vue {
  valid = true;
  id = "";
  idRules = [(v: string) => !!v || /.+@.+\..+/.test(v) || "E-MAIL is required"];
  pass = "";
  passRules = [(v: string) => !!v || "PASSWORD is required"];
  name = "";
  nameRules = [(v: string) => !!v || "NAME is required"];
  pass_again = "";
  pass_againRules = [(v: string) => v !== this.pass || "Don't match PASSWORD"];

  success = false;

  @Prop({ type: String, default: "" })
  given_id!: string;
  @Prop({ type: String, default: "" })
  givem_name!: string;

  // computed() {}
  mounted() {
    this.id = this.given_id;
    this.name = this.givem_name;
  }

  // ユーザーページを作ったらそこに飛ばす
  async trySignUp() {
    if (this.id !== "" && this.name !== "") {
      this.success = await signup_user(this.name, this.id, this.pass);
    } else {
      this.success = await edit_user(this.name, this.id, this.pass);
    }
  }
}
</script>