import { Home, User, ChevronDown } from "lucide-react"
import { Link, useLocation } from "react-router";
 
import {
  Sidebar,
  SidebarContent,
  SidebarGroup,
  SidebarGroupContent,
  SidebarGroupLabel,
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
  SidebarTrigger,
  SidebarHeader
} from "~/components/ui/sidebar";

import {
  DropdownMenu,
  DropdownMenuTrigger,
  DropdownMenuContent,
  DropdownMenuItem
} from "~/components/ui/dropdown-menu";
 
import { useTranslation } from "react-i18next";
import { routes } from "~/lib/routes";

 
export function AppSidebar({ children }: { children: React.ReactNode }) {
  const { t } = useTranslation();
  const location = useLocation();

  return (
    <>
    <Sidebar>

      <SidebarHeader>
        <SidebarMenu>
          <SidebarMenuItem>
            <DropdownMenu>
              <DropdownMenuTrigger asChild>
                <SidebarMenuButton>
                {sessionStorage.getItem("user_email") ?? ""}
                  <ChevronDown className="ml-auto" />
                </SidebarMenuButton>
              </DropdownMenuTrigger>

              <DropdownMenuContent className="w-[--radix-popper-anchor-width]">
                <DropdownMenuItem>
                  <Link to="/account">{t('menu.account')}</Link>
                </DropdownMenuItem>
                <DropdownMenuItem>
                  <Link to="/logout">{t('menu.logout')}</Link>
                </DropdownMenuItem>
              </DropdownMenuContent>
            </DropdownMenu>
          </SidebarMenuItem>
        </SidebarMenu>
      </SidebarHeader>

      <SidebarContent>
        <SidebarGroup>
          <SidebarGroupContent>
            <SidebarMenu>
              {Object.values(routes).map((r) => (
                r.titleKey && (
                  <SidebarMenuItem key={r.titleKey}>
                    <SidebarMenuButton asChild>
                      <Link to={r.path}>
                        <r.icon />
                        <span className={r.path == location.pathname ? "font-bold" : ""}>{t(r.titleKey)}</span>
                      </Link>
                    </SidebarMenuButton>
                  </SidebarMenuItem>
                )
             ))}
            </SidebarMenu>
          </SidebarGroupContent>
        </SidebarGroup>
      </SidebarContent>
    </Sidebar>
    <main>
      <SidebarTrigger />
      {children}
    </main>
    </>
  )
}
