import Vue from 'vue';
import axios from 'axios';

import App from './App.vue';
import { router } from './router';
import store from './store'

// axios
axios.defaults.baseURL = 'http://localhost:4000/'
axios.defaults.crossDomain = true
axios.defaults.headers.common['Content-Type'] = 'application/json'

// vue-axios
import VueAxios from 'vue-axios';
Vue.use(VueAxios, axios);

new Vue({
  router,
  store,
  render: h => h(App),
}).$mount('#app');

Vue.config.productionTip = false;
