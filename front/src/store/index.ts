import Vue from 'vue'
import Vuex from 'vuex'

Vue.use(Vuex)

export default new Vuex.Store({
  state: {
    login_state: false,
    user_id: 0,
    user_name: "",
  },
  getters: {
  },
  mutations: {
    login_state2ok: function (state) {
      state.login_state = true;
    },
    login_state2no: function (state) {
      state.login_state = false;
    },
    set_user_id: function (state, id: number) {
      state.user_id = id;
    },
    set_user_name: function (state, name: string) {
      state.user_name = name;
    },
  },
  actions: {
    login_state_ok: function (context) {
      context.commit('login_state2ok');
    },
    login_state_no: function (context) {
      context.commit('login_state2no');
    },
    set_user_id: function (context, id: number) {
      context.commit('set_user_id', id);
    },
    set_user_name: function (context, name: string) {
      context.commit('set_user_name', name);
    },
  },
  modules: {
  }
})
