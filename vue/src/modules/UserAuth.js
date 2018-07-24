import Vue from 'vue';
import JwtDecode from 'jwt-decode';
import qs from "qs";
import { router } from '../router'

const token = localStorage.getItem('token');
const initialState = token
  ? { currentJWT: token, loggedIn: true }
  : { currentJWT: '', loggedIn: false };

export default {
  name: "UserAuth",
  state: initialState,

  getters: {
    token: state => state.currentJWT,
    tokenData: state => state.currentJWT ? JwtDecode(state.currentJWT) : null,
    loggedIn: state => state.loggedIn,
  },

  mutations: {
    loginSuccess(state, token) {
      localStorage.setItem('token', token);
      state.currentJWT = token;
      state.loggedIn = true;
    },
    loginFailure(state, error) {
      localStorage.removeItem('token');
      state.currentJWT = null;
      state.loggedIn = false;
    }
  },

  actions: {
    login({ commit }, { username, password }) {
      Vue.axios({
        url: "/authenticate",
        method: "POST",
        headers: { 'Content-Type': 'application/json' },
        data: qs.stringify({
          username: username,
          password: password
        }),
        responseType: "json",
        responseEncoding: "utf8"
      })
        .then(response => {
          if (response.status == 200) {
            const token = response.data.token;
            commit('loginSuccess', token);
            router.push('/')
          }
        })
        .catch(error => {
          commit('loginFailure', error);
        });
    }
  }
}
