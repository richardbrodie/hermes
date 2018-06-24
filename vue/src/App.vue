<template>
  <div id="app">
    <sidebar></sidebar>
    <router-view></router-view>
  </div>
</template>

<script>
import Sidebar from "./components/Sidebar";
import FeedItems from "./components/FeedItems";

export default {
  name: "App",
  components: {
    Sidebar,
    FeedItems
  }
};
</script>

<style lang="scss">
#app {
  font-family: "Avenir", Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: left;
  color: #2c3e50;
}
</style>
