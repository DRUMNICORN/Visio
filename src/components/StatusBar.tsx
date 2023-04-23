import React, { useState } from 'react';
import { Box } from '@mui/material';
import SideBarIcons from './SideBarIcons';

const SideBar: React.FC = () => {
  const [activeIcon, setActiveIcon] = useState('fileTree');

  const handleIconClick = (icon: string) => {
    setActiveIcon(icon);
  };

  return (
    <Box
      sx={{
        width: '7%',
        backgroundColor: '#252526',
        color: 'white',
        overflowY: 'auto',
      }}
    >
      <SideBarIcons activeIcon={activeIcon} onIconClick={handleIconClick} />
    </Box>
  );
};

export default SideBar;
