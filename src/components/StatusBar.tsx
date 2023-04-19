import React from 'react';
import { Box } from '@mui/material';

const StatusBar: React.FC = () => {
  return (
    <Box
      sx={{
        height: '24px',
        backgroundColor: '#333',
        color: 'white',
        display: 'flex',
        alignItems: 'center',
        paddingLeft: '16px',
      }}
    >
      {/* Add your status bar content here */}
    </Box>
  );
};

export default StatusBar;
