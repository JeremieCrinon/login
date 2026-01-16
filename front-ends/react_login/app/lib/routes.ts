import {Home, User, type LucideIcon} from "lucide-react";

export interface RouteDefinition {
  path: string;
  titleKey?: string; // Keep empty if the route should not be displayed in the sidebar
  icon?: LucideIcon; // Keep empty if the route should not be displayed in the sidebar
  requiredRole?: string; // Keep empty if the route should not be displayed in the sidebar, this is just to not display the routes the user doesn't have the role to go to in the sidebar
  component: string; 
}

export const routes: Record<string, RouteDefinition> = {
  origin: {
    path: "/",
    component: "./pages/origin/origin.ts" // Follow the path from ../routes.ts, not from this file
  },
  login: {
    path: "/login",
    component: "./pages/login/login/login.tsx"
  },
  logout: {
    path: "/logout",
    component: "./pages/login/logout/logout.ts"
  },
  newAccount: {
    path: "/new-account",
    component: "./pages/login/new-account/newAccount.tsx"
  },
  verifyEmail: {
    path: "/verify-email",
    component: "./pages/login/verify-email/verifyEmail.tsx"
  },
  home: {
    path: "/home",
    titleKey: "menu.home",
    icon: Home,
    requiredRole: "user",
    component: "./pages/home/home.tsx"  
  },
  users: {
    path: "/users",
    titleKey: "menu.users",
    icon: User,
    requiredRole: "edit_users",
    component: "./pages/users/list/users.tsx"
  },
  createUser: {
    path: "/users/create",
    component: "./pages/users/create/createUser.tsx"
  },
  editUser: {
    path: "/users/:id",
    component: "./pages/users/edit/editUser.tsx"
  },
  account: {
    path: "/account",
    component: "./pages/account/account.tsx"
  }
}
