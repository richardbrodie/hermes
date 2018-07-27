<template>
  <div id="top-bar">
    <div class="menuMainItem"><a href="/">Feeds.rs</a></div>
    <div class="menuItem"><button type="button" id="add-feed-btn" class="btn-outline">add feed</button></div>
  </div>
</template>

<script>
export default {
  name: "TopBar"
};
</script>

<style lang="scss">
$sans-fonts: "Open Sans", sans-serif;
#top-bar {
  grid-row: 1;
  grid-column: 1 / span 2;
  padding: 0.5em;
  background-color: #e2e1e1;
}
.menuItem,
.menuMainItem {
  font-family: $sans-fonts;
  width: 150px;
  text-transform: uppercase;
  float: left;
  text-align: center;
  cursor: pointer;
}
.menuMainItem {
  font-size: 1.3em;
}
</style>
