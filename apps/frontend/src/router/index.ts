import { createRouter, createWebHistory } from 'vue-router'
import { useAuthStore } from '@/stores/auth'

const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: '/',
      name: 'home',
      component: () => import('@/views/HomeView.vue'),
    },
    {
      path: '/login',
      name: 'login',
      component: () => import('@/views/LoginView.vue'),
      meta: { guest: true },
    },
    {
      path: '/articles',
      name: 'articles',
      component: () => import('@/views/ArticlesView.vue'),
    },
    {
      path: '/articles/new',
      name: 'create-article',
      component: () => import('@/views/CreateArticleView.vue'),
      meta: { requiresAuth: true, roles: ['super_admin', 'admin', 'editor'] },
    },
    {
      path: '/articles/:id',
      name: 'article-detail',
      component: () => import('@/views/ArticleDetailView.vue'),
    },
    {
      path: '/articles/:id/edit',
      name: 'edit-article',
      component: () => import('@/views/EditArticleView.vue'),
      meta: { requiresAuth: true, roles: ['super_admin', 'admin', 'editor'] },
    },
    {
      path: '/favorites',
      name: 'favorites',
      component: () => import('@/views/FavoritesView.vue'),
      meta: { requiresAuth: true },
    },
    {
      path: '/search',
      name: 'search',
      component: () => import('@/views/SearchView.vue'),
    },
    {
      path: '/admin',
      name: 'admin',
      component: () => import('@/views/admin/AdminDashboard.vue'),
      meta: { requiresAuth: true, roles: ['super_admin', 'admin'] },
    },
    {
      path: '/admin/users',
      name: 'admin-users',
      component: () => import('@/views/admin/UsersView.vue'),
      meta: { requiresAuth: true, roles: ['super_admin', 'admin'] },
    },
    {
      path: '/admin/audit',
      name: 'admin-audit',
      component: () => import('@/views/admin/AuditView.vue'),
      meta: { requiresAuth: true, roles: ['super_admin', 'admin'] },
    },
  ],
})

// Navigation guard
router.beforeEach(async (to, from, next) => {
  const authStore = useAuthStore()

  // Try to fetch user if we don't have one
  if (!authStore.user && !authStore.loading) {
    await authStore.fetchUser()
  }

  const requiresAuth = to.meta.requiresAuth as boolean
  const allowedRoles = to.meta.roles as string[] | undefined
  const isGuest = to.meta.guest as boolean

  // Redirect logged-in users away from guest pages
  if (isGuest && authStore.isAuthenticated) {
    next({ name: 'home' })
    return
  }

  // Check authentication
  if (requiresAuth && !authStore.isAuthenticated) {
    next({ name: 'login', query: { redirect: to.fullPath } })
    return
  }

  // Check role permissions
  if (allowedRoles && authStore.user) {
    if (!allowedRoles.includes(authStore.user.role)) {
      next({ name: 'home' })
      return
    }
  }

  next()
})

export default router
