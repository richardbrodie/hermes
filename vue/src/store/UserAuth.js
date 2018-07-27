import JwtDecode from 'jwt-decode';
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
      var url = "http://localhost:4000/authenticate";
      var body = JSON.stringify({
        username: username,
        password: password
      });
      var headers = new Headers({
        "Content-Type": "application/json"
      });
      var req = new Request(url, {
        method: "POST",
        body: body,
        headers: headers
      });
      fetch(req)
        .then(resp => resp.json())
        .then(function (data) {
          commit('loginSuccess', data.token);
          router.push('/')
        }).catch(error => {
          commit('loginFailure', error);
        });
    }
  }
}
