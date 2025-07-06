import {
  Sidebar,
  SidebarContent,
  SidebarGroup,
  SidebarGroupContent,
  SidebarGroupLabel,
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
} from "./ui/sidebar"

export type SidebarSection = "presets" | "paths" | "mods"

interface AppSidebarProps {
  section: SidebarSection
  setSection: (section: SidebarSection) => void
}

const items = [
  {
    title: "presets",
    section: "presets",
  },
  {
    title: "paths",
    section: "paths",
  },
  {
    title: "mods",
    section: "mods"
  }
] as const

export function AppSidebar({ section, setSection }: AppSidebarProps) {
  return (
    <Sidebar>
      <SidebarContent>
        <SidebarGroup>
          <SidebarGroupContent>
            <SidebarMenu>
              {items.map((item) => (
                <SidebarMenuItem key={item.section}>
                  <SidebarMenuButton
                    isActive={section === item.section}
                    onClick={() => setSection(item.section)}
                  >
                    {item.title}
                  </SidebarMenuButton>
                </SidebarMenuItem>
              ))}
            </SidebarMenu>
          </SidebarGroupContent>
        </SidebarGroup>
      </SidebarContent>
    </Sidebar>
  )
}