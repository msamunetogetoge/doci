import Vue from 'vue'
import Vuex from 'vuex'

Vue.use(Vuex)

export default new Vuex.Store({
  state: {
    login_state: false,
  },
  getters: {
  },
  mutations: {
    login_state2ok: function (state) {
      state.login_state = true;
    },
    login_state2no: function (state) {
      state.login_state = false;
    }
  },
  actions: {
    login_state_ok: function (context) {
      context.commit('login_state2ok');
    },
    login_state_no: function (context) {
      context.commit('login_state2no');
    }
  },
  modules: {
  }
})
