<template>
  <div id='app'>
    <jwt/>
    <!-- <sidebar/>
    <router-view/> -->
  </div>
</template>

<script>
import Sidebar from './components/Sidebar';
import FeedItems from './components/FeedItems';
import Jwt from './components/Jwt';

export default {
  name: 'App',
  components: {
    Sidebar,
    FeedItems,
    Jwt
  }
};
</script>

<style lang="scss">
html {
  background: white;
  // text-rendering: optimizeLegibility;
  text-rendering: geometricPrecision;
  font-family: 'Open Sans', sans-serif;
  font-feature-settings: 'dlig', 'liga', 'lnum', 'kern';
  line-height: 1.5em;
}
body {
  height: 100vh;
  margin: 0;
}
#app {
  font-family: 'Avenir', Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: left;
  color: #2c3e50;

  display: grid;
  grid-template-columns: 200px auto;
  grid-column-gap: 10px;
}
</style>
