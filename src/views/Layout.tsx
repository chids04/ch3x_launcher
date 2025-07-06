import { ReactNode, useState } from "react";
import { SidebarProvider, SidebarTrigger } from "../components/ui/sidebar";
import { AppSidebar, SidebarSection } from "../components/AppSidebar";

interface LayoutProps {
  children: (section: SidebarSection, setSection: (section: SidebarSection) => void) => ReactNode;
}

export default function Layout({ children }: LayoutProps) {
  const [section, setSection] = useState<SidebarSection>("presets");
  
  return (
    <SidebarProvider>
      <div className="flex w-screen h-screen"> 
        <AppSidebar section={section} setSection={setSection} />
        <main className="flex-1 m-4">
          <SidebarTrigger />
          {children(section, setSection)}
        </main>
      </div>
    </SidebarProvider>
  );
}