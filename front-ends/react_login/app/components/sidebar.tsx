import { Home, User, ChevronDown } from "lucide-react"
import { Link } from "react-router";
 
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

// TODO: Use a centralized system between the router and menu
const items = [
  {
    title: "Home",
    url: "/home",
    icon: Home,
  },
  {
    title: "Users",
    url: "/users",
    icon: User,
  },
]
 
export function AppSidebar({ children }: { children: React.ReactNode }) {
  const { t } = useTranslation();
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
                  <Link to="/account">Account</Link>
                </DropdownMenuItem>
                <DropdownMenuItem>
                  <Link to="/logout">Logout</Link>
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
              {items.map((item) => (
                <SidebarMenuItem key={item.title}>
                  <SidebarMenuButton asChild>
                    <Link to={item.url}>
                      <item.icon />
                      <span>{item.title}</span>
                    </Link>
                  </SidebarMenuButton>
                </SidebarMenuItem>
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
