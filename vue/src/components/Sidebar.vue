<template>
  <div>
    <table>
      <tbody v-for="feed in feeds" v-bind:key='feed.id'>
        <tr>
          <td><router-link :to="{ path: '/feed/'+feed.id }">{{ feed.title }}</router-link></td>
          <td></td>
        </tr>
      </tbody>
    </table>
  </div>
</template>

<script>
export default {
  data() {
    return {
      feeds: []
    };
  },
  mounted() {
    this.axios("http://localhost:3000/feeds", {
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
