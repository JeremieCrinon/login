<script setup lang="ts">
import { useRouter } from 'vue-router';

// ShadCn imports
import { SidebarProvider, Sidebar, SidebarContent, SidebarHeader, SidebarGroup, SidebarTrigger } from '@/components/ui/sidebar';
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu';
import { Button } from '@/components/ui/button';

// Lucide icons
import { User } from 'lucide-vue-next';

import { useI18n } from 'vue-i18n';
import { useRoute } from 'vue-router'

const { t } = useI18n();
const router = useRouter();
const location = useRoute();

let routesToDisplay = router.options.routes; // Get the list of all the routes from the router
const userRoles = JSON.parse(sessionStorage.getItem('roles') || '[]'); // Get user roles from sessionStorage

// Filter all the routes to get only the ones the user has access to
routesToDisplay = routesToDisplay.filter((route) => {
    return route.meta && route.meta.menuName && (userRoles.includes(route.meta.requiresRole) || userRoles.includes('admin') || route.meta.requiresRole == "user"); // If the route has a menuName (the i18n identifier), if it don't, it means that the route should not be displayed in the menu. Also verifying that the user has the required role to see the route.
})

const userEmail = sessionStorage.getItem("user_email") ?? ""; // Get the user email to display on the button on the sidebar header
</script>

<template>
  <SidebarProvider>
        <div class="flex">

            <!-- The sidebar itslef -->
            <Sidebar>
                
                <!-- The top of the sidebar -->
                <SidebarHeader>
                    <DropdownMenu>
                        
                        <!-- The button with the user icon on top of the sidebar with a dropdown menu to edit accounts and logout -->
                        <DropdownMenuTrigger>
                            <div class="mt-2">
                                <Button>
                                    <div class="flex items-center">
                                        <User /> <!-- The user icon from lucide -->
                                        <p class="ml-2">{{ userEmail }}</p>
                                    </div>
                                </Button>
                            </div>
                        </DropdownMenuTrigger>
                        
                        <!-- The content of the dropdown menu -->
                        <DropdownMenuContent>
                            <RouterLink to="/modify-account">
                                <DropdownMenuItem>{{ t("menu.modify_account") }}</DropdownMenuItem>
                            </RouterLink>
                            <DropdownMenuSeparator />
                            <RouterLink to="/logout">
                                <DropdownMenuItem>{{ t("menu.disconnect") }}</DropdownMenuItem>
                            </RouterLink>
                        </DropdownMenuContent>

                    </DropdownMenu>
                </SidebarHeader>
                
                <!-- The content of the sidebar -->
                <SidebarContent>
                    <SidebarGroup>
                        <ul>
                            <li v-for="route in routesToDisplay" :key="route.path">
                                <RouterLink :to="route.path">
                                    <p :class="location.path === route.path ? 'font-bold' : ''" class="ml-5 mt-2 hover:opacity-75">{{ $t(route.meta?.menuName as string || "") }}</p>
                                </RouterLink>
                            </li>
                        </ul>
                    </SidebarGroup>
                </SidebarContent>
            </Sidebar>

            <SidebarTrigger />

            <!-- The content of the page -->
            <div class="flex-1">
                <slot />
            </div>
        </div>
  </SidebarProvider>
</template>
