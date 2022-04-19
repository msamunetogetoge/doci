<template>
  <v-app-bar app color="primary" dark clipped-left>
    <!-- <page-path-input v-if ="IsInput" @EndInput="IsInput = false" @GivePagePath="SetPagePath"/> -->
    <div class="d-flex align-center">
      <!-- doci icon <v-img
          alt="Vuetify Logo"
          class="shrink mr-2"
          contain
          src="https://cdn.vuetifyjs.com/images/logos/vuetify-logo-dark.png"
          transition="scale-transition"
          width="40"
        /> -->

      Doci
    </div>

    <v-spacer></v-spacer>
    <v-dialog
      v-model="dialog"
      persistent
      max-width="600px"
    >
      <template v-slot:activator="{ on, attrs }">
        <v-btn
        icon @click="IsInput = true"
          v-bind="attrs"
          v-on="on"
        >
          New 
        </v-btn>
      </template>
      <v-card>
        <v-card-title>
          <span class="text-h5">Input Page Path</span>
        </v-card-title>
        <v-card-text>
          <v-container>
            <v-row>
              <v-col
                cols="12"
                sm="6"
                md="4"
              >
                <v-text-field
                  label="app name/ 以下を入力してください。"
                  required
                  v-model="page_path"
                ></v-text-field>
              </v-col>
            </v-row>
          </v-container>
          <small>*indicates required field</small>
        </v-card-text>
        <v-card-actions>
          <v-spacer></v-spacer>
          <v-btn
            color="blue darken-1"
            text
            @click="dialog = false"
          >
            Close
          </v-btn>
          <v-btn
            color="blue darken-1"
            text
            @click="CheckPagePath"
          >
            Ok
          </v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>
    <!-- <v-btn icon @click="IsInput = true">
      <v-icon>mdi-pen-plus</v-icon>
      New
    </v-btn> -->
    <v-btn href="" target="_blank" icon>
      <v-icon>mdi-github</v-icon>
    </v-btn>
    <v-btn href="" target="_blank" text>
      <span class="mr-2">Login</span>
      <v-icon>mdi-open-in-new</v-icon>
    </v-btn>
  </v-app-bar>
   
</template>
<script lang="ts">
import { Vue, Component,Prop } from "vue-property-decorator";
import {IsExistPage,} from "../utils/page-util";

@Component
export default class AppBar extends Vue {
  dialog=false;
  @Prop({type:Boolean, default: false})
  IsInput?: boolean;
  
  @Prop({type:String, default:"/"})
  PagePath!:string;

  @Prop({type:Number, default:0})
  AppId!: number;

  @Prop({type:String, default:"app"})
  AppName!: string;

  page_path ="";
  mounted(){
    this.page_path = this.AppName+"/";
  }
  SetPagePath(page_path:string){
    const regx = new RegExp('/+', 'i');
    this.page_path= page_path.replaceAll(regx,"/");
  }

  async CheckPagePath(){
    let is_exist = await IsExistPage(this.AppId, this.page_path);
    if (is_exist){
      alert(this.page_path + " はすでに存在します")
    }else{
      this.$emit("GivePagePath", this.page_path)
      this.$emit("New", true);
      this.dialog=false
    }
  }
}


</script>
