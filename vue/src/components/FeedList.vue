<template>
  <div id='feed-list'>
    <table>
      <tbody v-for="feed in feeds" v-bind:key='feed.id'>
        <tr>
          <td><router-link :to="{ path: '/feeds/'+feed.id }">{{ feed.title }}</router-link></td>
          <td></td>
        </tr>
      </tbody>
    </table>
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
    this.axios({
      url: "/feeds",
      method: "GET",
      responseType: "json",
      responseEncoding: "utf8"
    })
      .then(response => {
        this.feeds = response.data;
      })
      .catch(err => {
        console.log(err);
      });
  }
};
</script>

<style>
a {
  color: white;
  text-decoration: none;
  outline: 0;
}
#feed-list {
  background: #2a2b2f;
  color: white;
}
</style>
