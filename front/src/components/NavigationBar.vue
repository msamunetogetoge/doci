<template>
  <v-card>
    <v-navigation-drawer
      app
      clipped
      v-model="drawer"
      :mini-variant.sync="mini"
      permanent
    >
    <v-btn depressed @click="CloseAllTree"> 閉じる</v-btn>
      <v-list-item class="px-2">
        <v-list-item-avatar>
          <v-icon>mdi-account-box</v-icon>
        </v-list-item-avatar>

        <v-list-item-title>UserName</v-list-item-title>

        <v-btn icon @click.stop="mini = !mini">
          <v-icon>mdi-chevron-left</v-icon>
        </v-btn>
      </v-list-item>

      <v-divider></v-divider>

      <v-list dense>
        <v-list-item v-for="item in items" :key="item.title" link>
          <v-list-item-icon>
            <v-icon>{{ item.icon }}</v-icon>
          </v-list-item-icon>

          <v-list-item-content>
            <v-list-item-title>{{ item.title }}</v-list-item-title>
          </v-list-item-content>
        </v-list-item>
      </v-list>
      <template>
        <v-treeview
          :items="items_folder"
          :dense="true"
          :open="open"
          @click="mini = !mini"
          :transition="true"
          :load-children="AddChildren"
        >
          <template v-slot:prepend="{ item }">
            <v-icon
              v-text="GetIcon(item)"
              
            ></v-icon>

          </template>
        </v-treeview>
      </template>
    </v-navigation-drawer>
  </v-card>
</template>
<script lang="ts">
import { Vue, Component , Prop, Watch} from "vue-property-decorator";
import {GetFolders,Hierarchy } from "../utils/hierarchy-utils";

@Component
export default class NavBar extends Vue {
  @Prop({type:Number, default:0})
  AppId!:number;

  @Prop({type:String, default:"app"})
  AppName!:string;

  @Prop({type:Number, default:0})
  UserId!:number;

  @Prop({ default:() =>[]})
  PageHierarchy!:Hierarchy[];



  drawer = true;
  mini = true;
  // treeviewで、最初に開いておくフォルダなど
  open =[];
  
  //アイコン名
  files = {
    folder:"mdi-folder",
    folders:"mdi--file-document-multiple",
    html: "mdi-language-html5",
    js: "mdi-nodejs",
    json: "mdi-code-json",
    md: "mdi-language-markdown",
    pdf: "mdi-file-pdf",
    png: "mdi-file-image",
    txt: "mdi-file-document-outline",
    xls: "mdi-file-excel",
  };
  
  items_folder:Hierarchy[]=[];// tree-view の中身

 // mount が終わったらプロパティの初期値をtree-view にセットする
  mounted(){
  // this.items_folder=this.PageHierarchy;
  this.items_folder=[{app_id:this.AppId, name: this.AppName,depth:1,id:undefined, children:[]}];
 }
 

  // tree-view のアイコンを決める関数
  GetIcon(item:Hierarchy):string{
    if (item.depth === 1 ){
      return this.files["folders"];
    }else if(item.children){
      return this.files["folder"];
    }else{
        return this.files["md"];
      }
  }

  // tree-view のアイコンをクリックしたら、フォルダの中身を検索して追加する関数 
  async AddChildren(parent: Hierarchy){
    let child:Hierarchy[] = await GetFolders(this.AppId, parent.depth, parent.name, parent.id);
    
    if (parent.children){
      for (let index = 0; index < child.length; index++) {
        const element = child[index];
        parent.children.push(element);
        
      }
    }
    else{
      // 何もしない
    }

  }

}
</script>

