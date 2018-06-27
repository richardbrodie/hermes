<template>
  <div id='sidebar'>
    <vue-tree-navigation :items='items' />
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
    this.axios("http://localhost:3000/feeds", {
      method: "GET",
      crossDomain: true,
      responseType: "json",
      responseEncoding: "utf8"
    }).then(response => {
      response.data.forEach(e =>
        this.items.push({ name: e.title, route: "/feed/" + e.id })
      );
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
.NavigationItem {
}
.NavigationItem--active {
  background-color: rgba(255, 255, 255, 0.15);
  margin-left: -17px;
  padding-left: 17px;
  margin-right: -23px;
  padding-right: 23px;
}
</style>
