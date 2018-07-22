<template>
  <div class="form">
    <h2 class="form-signin-heading">Please sign in</h2>
    <div class="alert alert-danger" v-if="error">{{ error }}</div>
    <form class="login-form" @submit.prevent="login">
      <input v-model="username" type="text" placeholder="username"/>
      <input v-model="password" type="password" placeholder="password"/>
      <button>login</button>
    </form>
  </div>
</template>


<script>
import qs from "qs";
export default {
  name: "Login",
  data() {
    return {
      username: "",
      password: "",
      error: false
    };
  },
  updated() {
    this.checkCurrentLogin();
  },
  created() {
    this.checkCurrentLogin();
  },
  methods: {
    checkCurrentLogin() {
      if (localStorage.token) {
        this.$router.replace("/feeds");
      }
    },
    login() {
      this.$http({
        url: "/authenticate",
        method: "POST",
        data: qs.stringify({
          username: this.username,
          password: this.password
        }),
        responseType: "json",
        responseEncoding: "utf8"
      })
        .then(request => this.loginSuccessful(request))
        .catch(() => this.loginFailed());
    },

    loginSuccessful(req) {
      if (!req.data.token) {
        this.loginFailed();
        return;
      }

      localStorage.token = req.data.token;
      this.error = false;

      this.$router.replace("/feeds");
    },

    loginFailed() {
      this.error = "Login failed!";
      delete localStorage.token;
    }
  }
};
</script>


<style>
.form {
  width: 360px;
  margin: 0 auto 100px;
  position: relative;
  z-index: 1;
  background: #ffffff;
  max-width: 360px;
  padding: 45px;
  text-align: center;
  box-shadow: 0 0 20px 0 rgba(0, 0, 0, 0.2), 0 5px 5px 0 rgba(0, 0, 0, 0.24);
}
.form input {
  font-family: "Roboto", sans-serif;
  outline: 0;
  background: #f2f2f2;
  width: 100%;
  border: 0;
  margin: 0 0 15px;
  padding: 15px;
  box-sizing: border-box;
  font-size: 14px;
}
.form button {
  font-family: "Roboto", sans-serif;
  text-transform: uppercase;
  outline: 0;
  background: #4caf50;
  width: 100%;
  border: 0;
  padding: 15px;
  color: #ffffff;
  font-size: 14px;
  -webkit-transition: all 0.3 ease;
  transition: all 0.3 ease;
  cursor: pointer;
}
.form button:hover,
.form button:active,
.form button:focus {
  background: #43a047;
}
body {
  background: -webkit-linear-gradient(right, #76b852, #8dc26f);
  background: -moz-linear-gradient(right, #76b852, #8dc26f);
  background: -o-linear-gradient(right, #76b852, #8dc26f);
  background: linear-gradient(to left, #76b852, #8dc26f);
}
</style>
