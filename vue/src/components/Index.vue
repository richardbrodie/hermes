<template>
  <div>
    <div v-for="feed in feeds" v-bind:key='feed.id'>
      <feed-channel v-bind:feed="feed"></feed-channel>
    </div>
  </div>
</template>

<script type = "text/javascript" >
import FeedChannel from "./FeedChannel";
import axios from "axios";

export default {
  components: {
    FeedChannel
  },
  data() {
    return {
      feeds: []
    };
  },
  mounted() {
    axios("http://localhost:3000/feeds", {
      method: "GET",
      crossDomain: true,
      responseType: "json",
      responseEncoding: "utf8"
    }).then(response => {
      this.feeds = response.data;
    });
  }
};
</script>

<style>
</style>
