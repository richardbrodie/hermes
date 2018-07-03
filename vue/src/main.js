import Vue from 'vue';

import VueRouter from 'vue-router';
Vue.use(VueRouter);

// import Vuex from 'vuex';
// Vue.use(Vuex);

import axios from 'axios';
import VueAxios from 'vue-axios';
Vue.use(VueAxios, axios);

import VueTreeNavigation from 'vue-tree-navigation';
Vue.use(VueTreeNavigation);

import App from './App.vue';
import router from './router';

// const store = new Vuex.Store({
//   state: {
//     count: 0
//   }
// });

new Vue({
  render: h => h(App),
  router
  // store
}).$mount('#app');

Vue.config.productionTip = false;
