import React from "react";
import { Box, IconButton } from "@mui/material";
import { FilterNone as FileTree, Search, Settings } from "@mui/icons-material";
// enum types for window location
export enum WindowLocation {
  left_top = "left_top",
  left_bot = "left_bot",
  right_top = "right_top",
  right_bot = "right_bot",
}

export interface Window {
  name: string;
  IconComponent: React.ComponentType;
  location: WindowLocation;
  content: React.ReactNode;
}

interface SideBarIconsProps {
  activeIcon: string;
  onIconClick: (icon: string) => void;
  windows: Window[];
}

const SideBarIcons: React.FC<SideBarIconsProps> = ({ activeIcon, onIconClick, windows }) => {
  return (
    <Box
      sx={{
        width: "7%",
        backgroundColor: "#252526",
        color: "white",
        display: "flex",
        flexDirection: "column",
        alignItems: "center",
        justifyContent: "space-between",
      }}
    >
      {windows.map(({ name, IconComponent }) => (
        <IconButton key={name} color={activeIcon === name ? "primary" : "inherit"} onClick={() => onIconClick(name)}>
          <IconComponent />
        </IconButton>
      ))}
    </Box>
  );
};

export default SideBarIcons;
