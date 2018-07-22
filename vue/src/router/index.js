import Vue from 'vue';
import Router from 'vue-router';
import FeedItems from '@/components/FeedItems';
import FeedView from '@/components/FeedView';
import Login from '@/components/Login';

Vue.use(Router);

export default new Router({
  routes: [
    {
      path: '/',
      name: 'Login',
      component: Login
    },
    {
      path: '/feeds',
      name: 'Feeds',
      component: FeedView,
      children: [
        {
          path: ':id',
          name: 'FeedItems',
          component: FeedItems
        },
      ]
    }
  ]
});
