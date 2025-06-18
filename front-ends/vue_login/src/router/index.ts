import { createRouter, createWebHistory } from 'vue-router'

import LoginView from '@/views/login/Login/LoginView.vue'
import OriginView from '@/views/OriginView.vue'
import NewAccountView from '@/views/login/NewAccountView.vue'
import LogoutView from '@/views/login/LogoutView.vue'
import VerifyEmailView from '@/views/login/VerifyEmailView.vue'
import NotFoundView from '@/views/errors/NotFoundView.vue'
import ResetPassword from '@/views/login/ResetPassword.vue'
import HomeView from '@/views/app_pages/Home/HomeView.vue'
import UsersView from '@/views/app_pages/Users/UsersView.vue'
import ModifyAccountView from '@/views/app_pages/ModifyAccount/ModifyAccountView.vue'
import PrivacyView from '@/views/legal/PrivacyView.vue'
import CreditsView from '@/views/legal/CreditsView.vue'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    // The origin, this route redirects the user to different places depending on where he should be, and verifies the user's token is valid
    {
      path: '/',
      name: 'origin',
      component: OriginView,
    },
    // Login system pages
    {
      path: '/login',
      name: 'login',
      component: LoginView
    },
    {
      path: '/forgot-password/:code',
      name: 'forgot_password',
      component: ResetPassword
    },
    {
      path: '/logout',
      name: 'logout',
      component: LogoutView
    },
    {
      path: '/new-account',
      name: 'new_account',
      component: NewAccountView
    },
    {
      path: '/verify-email',
      name: 'verify_email',
      component: VerifyEmailView
    },
    // Error pages
    {
      path: '/:notfound',
      name: 'not_found',
      component: NotFoundView
    },
    // The app pages
    {
      path: '/modify-account',
      name: 'modify_account',
      component: ModifyAccountView
    },
    {
      path: '/home',
      name: 'home',
      component: HomeView,
      meta: {requiresRole: 'user', menuName: 'menu.home'}
    },
    {
      path: '/users',
      name: 'users',
      component: UsersView,
      meta: {requiresRole: 'edit_users', menuName: 'menu.users'}
    },
    // Legal pages
    {
      path: '/privacy',
      name: 'privacy',
      component: PrivacyView
    },
    {
      path: '/credits',
      name: 'credits',
      component: CreditsView
    }
  ],
})

// Navigation guard
router.beforeEach((to, from, next) => {
  const userRoles = JSON.parse(sessionStorage.getItem('roles') || '[]'); // Get user roles from sessionStorage
  
  // Check if the route requires a specific role
  if (to.meta.requiresRole) {
    const requiredRole = to.meta.requiresRole;

    if (!((userRoles.includes(requiredRole) || requiredRole === "user") || (userRoles.includes("admin") && !userRoles.includes("new_account") && !userRoles.includes("unverified_email")))) {
      next({ name: 'origin' }); // If the user does not have the required role and is not admin (or he is admin but hasn't verified his account), we redirect him to the origin
    } else if (userRoles.includes("new_account")) {
      console.log(2)
      next({ name: 'new_account' }); // If it's a new account, we redirect the user to the route to modify the default password
    } else if (userRoles.includes("unverified_email")) {
      next({name: 'verify_email'});
    } else {
      next();
    }
  } else {
    next(); // Route does not have role restrictions, proceed as normal
  }
});

export default router;
