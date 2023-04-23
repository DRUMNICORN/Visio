import React from "react";
import { Box, IconButton } from "@mui/material";
import { FilterNone as FileTree, Search, Settings } from "@mui/icons-material";

interface SideBarIconsProps {
  activeIcon: string;
  onIconClick: (icon: string) => void;
}

const SideBarIcons: React.FC<SideBarIconsProps> = ({ activeIcon, onIconClick }) => {
  const icons = [
    { name: "fileTree", IconComponent: FileTree },
    { name: "search", IconComponent: Search },
    { name: "settings", IconComponent: Settings },
  ];

  return (
    <Box
    sx={{
      width: '7%',
      backgroundColor: '#252526',
      color: 'white',
      // align items to the top
      // place 
    }}
    >
      {icons.map(({ name, IconComponent }) => (
        <IconButton key={name} color={activeIcon === name ? "primary" : "inherit"} onClick={() => onIconClick(name)}>
          <IconComponent />
        </IconButton>
      ))}
    </Box>
  );
};

export default SideBarIcons;
