import Vue from 'vue';
import VueRouter from 'vue-router';
Vue.use(VueRouter);

import axios from 'axios';
import VueAxios from 'vue-axios';
Vue.use(VueAxios, axios);

import VueTreeNavigation from 'vue-tree-navigation';
Vue.use(VueTreeNavigation);

import App from './App.vue';

import router from './router';

new Vue({
  render: h => h(App),
  router
}).$mount('#app');

Vue.config.productionTip = false;
