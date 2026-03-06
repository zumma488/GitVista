import { createRouter, createWebHashHistory } from 'vue-router'

const router = createRouter({
  history: createWebHashHistory(),
  routes: [
    {
      path: '/',
      name: 'projects',
      component: () => import('../views/ProjectListView.vue'),
    },
    {
      path: '/repo',
      name: 'repo',
      component: () => import('../views/RepoView.vue'),
    },
  ],
})

export default router
