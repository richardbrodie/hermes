<template>
  <div>
    <div v-bind:feed="feed">
      <h3>{{ feed.title }}</h3>
      <p>{{ feed.site_link }}</p>
      <p>{{ feed.description }}</p>
    </div>
    <p>
      <div v-for="item in items" v-bind:key='item.id'>
        <feed-item v-bind:item="item"></feed-item>
      </div>
    </p>
  </div>
</template>

<script type = "text/javascript" >
import FeedItem from "./FeedItem";
import axios from "axios";

export default {
  components: {
    FeedItem
  },
  props: {
    feed: Object
  },
  data() {
    return {
      items: []
    };
  },
  mounted() {
    axios("http://localhost:3000/items/1", {
      method: "GET",
      crossDomain: true,
      responseType: "json",
      responseEncoding: "utf8"
    }).then(response => {
      this.items = response.data;
    });
  }
};
</script>

<style>
</style>
