import Vue from 'vue';

import VueRouter from 'vue-router';
Vue.use(VueRouter);

import Vuex from 'vuex';
Vue.use(Vuex);

import axios from 'axios';
import VueAxios from 'vue-axios';
Vue.use(VueAxios, axios);

import App from './App.vue';
import router from './router';

const store = new Vuex.Store({
  state: {
    token:
      'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpYXQiOjE1MzA2NDc5MjUsIm5hbWUiOiJhZG1pbiJ9.SQmNZdzbqA7Hi9yO0GKXY6HRV-9cHBvCMUSENFl1zqY='
  }
});

new Vue({
  render: h => h(App),
  store,
  router
}).$mount('#app');

Vue.config.productionTip = false;
