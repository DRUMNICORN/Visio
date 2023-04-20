import React from 'react';
import { Box, Button} from '@mui/material';
import SideBar from './components/SideBar';
import EditorArea from './components/EditorArea';
import StatusBar from './components/StatusBar';

import { emit, listen } from '@tauri-apps/api/event'

function App() {
  return (
    <Box
      sx={{
        display: 'flex',
        flexDirection: 'column',
        height: '100vh',
        backgroundColor: '#1e1e1e',
      }}
    >
      <Box sx={{ flexGrow: 1, display: 'flex' }}>
        <SideBar />
        <EditorArea />
      </Box>
      <StatusBar />
    </Box>
  );
}

export default App;
