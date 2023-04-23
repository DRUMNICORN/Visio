import React from 'react';
import { Box } from '@mui/material';

const EditorArea: React.FC = () => {
  return (
    <Box
      component="main"
      sx={{
        flexGrow: 1,
        backgroundColor: '#1e1e1e',
        color: 'white',
        padding: '16px',
        overflowY: 'auto',
      }}
    >
      {/* Add your editor component here */}
    </Box>
  );
};

export default EditorArea;
