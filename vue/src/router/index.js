import Vue from 'vue';
import Router from 'vue-router';

import FeedItems from '@/components/FeedItems';
import FeedView from '@/components/FeedView';
import Login from '@/components/Login';
import store from '../store'

Vue.use(Router);

export const router = new Router({
  mode: 'history',
  routes: [
    { path: '/login', component: Login },
    {
      path: '/',
      component: FeedView,
      children: [
        { path: 'feed/:id', component: FeedItems },
      ]
    },
    { path: '*', redirect: '/' }
  ]
});

router.beforeEach((to, _from, next) => {
  const publicPages = ['/login'];
  const authRequired = !publicPages.includes(to.path);
  const loggedIn = store.getters.loggedIn

  if (authRequired && !loggedIn) {
    return next('/login');
  }

  next();
})
