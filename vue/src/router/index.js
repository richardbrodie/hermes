import Vue from 'vue'
import Router from 'vue-router'
import FeedItems from '@/components/FeedItems'

Vue.use(Router)

export default new Router({
  routes: [{
    path: '/feed/:id',
    name: 'FeedItems',
    component: FeedItems
  }]
})
