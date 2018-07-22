<template>
  <div id='feed-view'>
    <feed-list/>
    <router-view/>
  </div>
</template>

<script>
import FeedList from "./FeedList";
import FeedItems from "./FeedItems";
export default {
  name: "FeedView",
  components: {
    FeedList,
    FeedItems
  }
};
</script>

<style lang="scss">
#feed-view {
  display: grid;
  grid-template-columns: 200px auto;
  grid-column-gap: 10px;
}
</style>
