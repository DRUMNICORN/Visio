import React, { useState } from "react";
import { Box, Drawer } from "@mui/material";
import SideBarIcons, { Window, WindowLocation } from "./SideBarIcons";
import { FilterNone as FileTree, Search, Settings } from "@mui/icons-material";
import SideBarWindow from "./SideBarWindow";
import { Resizable } from "react-resizable";
import "react-resizable/css/styles.css";
import { useAppContext } from "../AppContext";
import ResizableDrawer from "./ResizableDrawer";

const SideBar: React.FC = () => {
  const { graphData, windows } = useAppContext();

  const [activeIcon, setActiveIcon] = React.useState("");
  const [isWindowOpen, setIsWindowOpen] = React.useState(false);
  const [drawerWidth, setDrawerWidth] = React.useState(200);
  const [selectedWindowContent, setSelectedWindowContent] = React.useState<React.ReactNode | null>(null);

  const handleIconClick = (icon: string) => {
    setActiveIcon(icon);
    const selectedWindow = windows.find((window) => window.name === icon);
    if (selectedWindow) {
      // if its same as the active window, close the window
      if (selectedWindow.name === activeIcon) {
        setIsWindowOpen(!isWindowOpen);
      } else {
        setSelectedWindowContent(selectedWindow.content);
        setIsWindowOpen(true);
      }
    }
  };

  const handleResize = (event: React.SyntheticEvent, data: { size: { width: number; height: number } }) => {
    console.log(data);
    setDrawerWidth(data.size.width);
  };

  return (
    <>
      <Box
        sx={{
          paddingTop: 1,
          paddingBottom: 1,
          paddingLeft: 1,
          paddingRight: 1,
          width: "5%",
          backgroundColor: "#252526",
          color: "white",
          display: "flex",
          justifyContent: "space-between",
          alignItems: "center",
          flexDirection: "column",
          zIndex: 2,
        }}
      >
        <SideBarIcons
          key="side-bar-top"
          activeIcon={activeIcon}
          onIconClick={handleIconClick}
          windows={windows.filter((window) => window.location === WindowLocation.left_top)}
        />
        <SideBarIcons
          key="side-bar-bot"
          activeIcon={activeIcon}
          onIconClick={handleIconClick}
          windows={windows.filter((window) => window.location === WindowLocation.left_bot)}
        />
      </Box>
      <ResizableDrawer
        open={isWindowOpen}
        onClose={() => setIsWindowOpen(false)}
        drawerWidth={drawerWidth}
        onResize={handleResize}
      >
        <SideBarWindow content={selectedWindowContent} />
      </ResizableDrawer>
    </>
  );
};

export default SideBar;
