<template>
  <div id='feed-items' >
    <div class='feed-item' v-for='item in items' v-bind:key='item.id'>
      <div class='content'>
        <div class='title'>{{ item.title }}</div>
        <div class='desc'>{{ item.description }}</div>
      </div>
      <div class='pub_date'>{{ item.published_at }}</div>
    </div>
  </div>
</template>

<script type = 'text/javascript' >
export default {
  data() {
    return {
      items: []
    };
  },
  created() {
    this.fetchData();
  },
  watch: {
    $route: 'fetchData'
  },
  methods: {
    fetchData() {
      this.axios({
        url: 'http://localhost:4000/items/' + this.$route.params.id,
        method: 'GET',
        crossDomain: true,
        responseType: 'json',
        responseEncoding: 'utf8'
      }).then(response => {
        this.items = response.data;
      });
    }
  }
};
</script>

<style lang="scss">
.feed-item {
  display: grid;
  grid-template-columns: 7fr 1fr;
  grid-column-gap: 10px;
  border-bottom: 1px solid #e6e5e5;
}
.content {
  grid-column: 1 / span 1;
  .title {
    font-weight: 900;
  }
  .desc {
    font-weight: 100;
  }
}
.pub_date {
  grid-column: 2 / span 1;
}
</style>
