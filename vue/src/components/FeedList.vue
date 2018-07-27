<template>
  <div id='feed-list'>
    <div v-for="feed in feeds" v-bind:key='feed.id'>
      <router-link :to="{ path: '/feed/'+feed.id }">{{ feed.title }}</router-link>
    </div>
  </div>
</template>

<script>
export default {
  name: "FeedList",
  data() {
    return {
      feeds: []
    };
  },
  created() {
    this.fetchData();
  },
  methods: {
    fetchData() {
      const url = "http://localhost:4000/feeds";
      var headers = new Headers({
        "Content-Type": "application/json",
        Authorization: "Bearer " + this.$store.getters.token
      });
      var req = new Request(url, {
        method: "GET",
        headers: headers
      });
      fetch(req)
        .then(resp => resp.json())
        .then(data => (this.feeds = data))
        .catch(err => {
          console.log(err);
        });
    }
  }
};
</script>

<style lang="scss">
a {
  text-decoration: none;
  outline: 0;
}

#feed-list {
  padding-left: 0.5em;
  grid-row: 2;
  grid-column: 1;
  color: white;

  .router-link-active {
    font-weight: 600;
  }
}
</style>
