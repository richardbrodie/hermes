import { observable, computed } from 'mobx'
import JwtDecode from 'jwt-decode';

const store = observable({
  currentJWT: null,
  loggedIn: false,
  msgCallback: null,

  setToken(token) {
    if (token) {
      this.currentJWT = token ? token : null
      this.loggedIn = true
      localStorage.setItem('token', token);
    }
  },

  removeToken() {
    this.currentJWT = null
    this.loggedIn = false
    localStorage.removeItem('token');
  },

  get tokenData() {
    return this.currentJWT ? JwtDecode(this.currentJWT) : null
  },

  get accessToken() {
    return `?access_token=${this.currentJWT}`
  },

  setMsgCallback(handler) {
    this.msgCallback = handler
  }
})

export default store
