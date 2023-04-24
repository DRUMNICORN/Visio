// AppContext.tsx
import { FilterNone as FileTree, Search, Settings } from "@mui/icons-material";
import { createContext, useContext, useState, useEffect, ReactNode } from "react";
import { emit, listen } from "@tauri-apps/api/event";

interface AppContextType {
  graphData: any[];
  setGraphData: (data: any[]) => void;
  windows: Window[];
  setWindows: (windows: Window[]) => void;
}

// enum types for window location
enum WindowLocation {
  left_top = "left_top",
  left_bot = "left_bot",
  right_top = "right_top",
  right_bot = "right_bot",
}

interface Window {
  name: string;
  IconComponent: React.ComponentType;
  location: WindowLocation;
  content: React.ReactNode;
}

// export the window location enum

const AppContext = createContext<AppContextType | undefined>(undefined);

export function useAppContext() {
  const context = useContext(AppContext);
  if (!context) {
    throw new Error("useAppContext must be used within an AppProvider");
  }
  return context;
}

interface AppProviderProps {
  children: ReactNode;
}

export function AppProvider({ children }: AppProviderProps) {
  const [graphData, setGraphData] = useState<any[]>([]);
  const [windows, setWindows] = useState<Window[]>([
    {
      name: "fileTree",
      location: WindowLocation.left_top,
      IconComponent: FileTree,
      content: <div>File Tree Content</div>,
    },
    {
      name: "search",
      location: WindowLocation.left_top,
      IconComponent: Search,
      content: <div>Search Content</div>,
    },
    {
      name: "settings",
      location: WindowLocation.left_bot,
      IconComponent: Settings,
      content: <div>Settings Content</div>,
    },
  ]);

  useEffect(() => {
    const updateHandler = (event: any) => {
      if (event.section === "graphData") {
        setGraphData(event.data);
      } else if (event.section === "windows") {
        setWindows(event.data);
      }
    };

    listen("update", updateHandler);

    return () => {
      emit("removeListener", { event: "update", handler: updateHandler });
    };
  }, []);

  const value = {
    graphData,
    setGraphData,
    windows,
    setWindows,
  };

  return <AppContext.Provider value={value}>{children}</AppContext.Provider>;
}
