<template>
  <div id="jwt">
    <p>JWT: {{jwt}}</p>
  </div>
</template>

<script>
import qs from 'qs';
export default {
  data() {
    return {
      jwt: 'no token found'
    };
  },
  created() {
    const data = {
      username: 'admin',
      password: 'admin'
    };
    this.axios({
      url: 'http://localhost:4000/authenticate',
      method: 'POST',
      responseType: 'text',
      data: qs.stringify(data)
    }).then(response => {
      this.jwt = response.data;
    });
  }
};
</script>
