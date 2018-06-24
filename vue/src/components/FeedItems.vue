<template>
  <div>
    <table>
      <tbody v-for="item in items" v-bind:key='item.id'>
        <tr>
          <td>{{ item.title }}</td>
          <td>{{ item.description }}</td>
          <td>{{ item.published_at }}</td>
        </tr>
      </tbody>
    </table>
  </div>
</template>

<script type = "text/javascript" >
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
    $route: "fetchData"
  },
  methods: {
    fetchData() {
      this.axios("http://localhost:3000/items/" + this.$route.params.id, {
        method: "GET",
        crossDomain: true,
        responseType: "json",
        responseEncoding: "utf8"
      }).then(response => {
        this.items = response.data;
      });
    }
  }
};
</script>

<style>
</style>
