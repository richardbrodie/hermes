import Vue from 'vue';

import VueRouter from 'vue-router';
Vue.use(VueRouter);

import Vuex from 'vuex';
Vue.use(Vuex);

import axios from 'axios';
const API_URL = process.env.API_URL || 'http://localhost:4000/'
let axios_instance = axios.create({
  baseURL: API_URL,
  crossDomain: true,
  headers: {
    'Content-Type': 'application/json',
    'Authorization': 'Bearer ' + localStorage.token
  }
})
import VueAxios from 'vue-axios';
Vue.use(VueAxios, axios_instance);

import App from './App.vue';
import router from './router';

// const store = new Vuex.Store({
//   state: {
//     token:
//       'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpYXQiOjE1MzA2NDc5MjUsIm5hbWUiOiJhZG1pbiJ9.SQmNZdzbqA7Hi9yO0GKXY6HRV-9cHBvCMUSENFl1zqY='
//   }
// });

new Vue({
  render: h => h(App),
  // store,
  router
}).$mount('#app');

Vue.config.productionTip = false;
