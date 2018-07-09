<template>
  <div id='sidebar'>
    <div class='feed-item' v-for='item in items' v-bind:key='item.id'>
      <div class='content'>
        <div class='title'>{{ item.title }}</div>
        <div class='desc'>{{ item.description }}</div>
      </div>
      <div class='pub_date'>{{ item.published_at }}</div>
    </div>
  </div>
</template>

<script>
export default {
  data() {
    return {
      items: []
    };
  },
  created() {
    // console.log(this.$store.state.token);
    this.axios({
      url: 'http://localhost:4000/feeds',
      method: 'GET',
      crossDomain: true,
      responseType: 'json',
      responseEncoding: 'utf8',
      headers: { Authorization: this.$store.state.token }
      // headers: { Authorization: 'Bearer ' + this.$store.state.token }
    })
      .then(response => {
        response.data.forEach(e =>
          this.items.push({ name: e.title, route: '/feed/' + e.id })
        );
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
#sidebar {
  background: #2a2b2f;
  color: white;
}
/* .NavigationItem {
} */
.NavigationItem--active {
  background-color: rgba(255, 255, 255, 0.15);
  margin-left: -17px;
  padding-left: 17px;
  margin-right: -23px;
  padding-right: 23px;
}
</style>
