import Vue from 'vue';
import Vuex from 'vuex';
import UserAuth from '../modules/UserAuth'

Vue.use(Vuex);
export default new Vuex.Store({
  modules: {
    UserAuth
  }
})

