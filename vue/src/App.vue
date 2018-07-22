<template>
  <div id='app'>
    <router-view/>
  </div>
</template>

<script>
import FeedView from "./components/FeedView";
import Login from "./components/Login";

export default {
  name: "App",
  updated() {
    if (!localStorage.token && this.$route.path !== "/") {
      this.$router.push("/?redirect=" + this.$route.path);
    }
  },
  components: {
    FeedView,
    Login
  }
};
</script>

<style lang="scss">
html {
  background: white;
  text-rendering: geometricPrecision;
  font-family: "Open Sans", sans-serif;
  font-feature-settings: "dlig", "liga", "lnum", "kern";
  line-height: 1.5em;
}
body {
  height: 100vh;
  margin: 0;
}
#app {
  font-family: "Avenir", Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: left;
  color: #2c3e50;
}
</style>
