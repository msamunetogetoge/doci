import Vue from 'vue'
import VueRouter, { RouteConfig } from 'vue-router'
import DocView from '../views/DocView.vue'
import LoginView from '../views/LoginView.vue'
import store from '@/store/index';
import SignUpView from "../views/SignUpView.vue"

Vue.use(VueRouter)

const routes: Array<RouteConfig> = [
  {
    path: '/',
    name: 'login',
    component: LoginView
  },
  {
    path: '/signup',
    name: 'signup',
    component: SignUpView
  },
  {
    path: '/doc',
    name: 'doc',
    component: DocView
  },
  {
    path: '/about',
    name: 'about',
    // route level code-splitting
    // this generates a separate chunk (about.[hash].js) for this route
    // which is lazy-loaded when the route is visited.
    component: () => import(/* webpackChunkName: "about" */ '../views/AboutView.vue')
  }
]

const router = new VueRouter({
  mode: 'history',
  base: process.env.BASE_URL,
  routes
})

router.beforeEach((to, from, next) => {
  if ((to.name !== 'login' && to.name !== 'signup') && !store.state.login_state) {
    next({ name: 'login' })
  }
  else {
    next()
  }
})

export default router
