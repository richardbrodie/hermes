<template>
  <div id='feed-view'>
    <top-bar/>
    <feed-list/>
    <router-view/>
  </div>
</template>

<script>
import FeedList from "./FeedList";
import FeedItems from "./FeedItems";
import TopBar from "./TopBar";
export default {
  name: "FeedView",
  components: {
    FeedList,
    FeedItems,
    TopBar
  }
};
</script>

<style lang="scss">
#feed-view {
  display: grid;
  grid-template-columns: 200px 1fr;
  grid-column-gap: 10px;
}
</style>
