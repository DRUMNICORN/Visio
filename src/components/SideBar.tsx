import React from 'react';
import { Box } from '@mui/material';

const SideBar: React.FC = () => {
  return (
    <Box
      sx={{
        width: '7%',
        backgroundColor: '#252526',
        color: 'white',
        overflowY: 'auto',
      }}
    >
      {/* Add your file tree or other sidebar content here */}
    </Box>
  );
};

export default SideBar;
