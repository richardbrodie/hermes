<template>
  <div v-bind:item="item">
    <h5>{{ item.title }}</h5>
    <p>{{ item.published_at }}</p>
    <p>{{ item.description }}</p>
  </div>
</template>

<script type = "text/javascript" >
export default {
  props: {
    item: Object
  }
};
</script>

<style>
</style>
