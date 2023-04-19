import React from 'react';
import { Box, Button} from '@mui/material';
import SideBar from './components/SideBar';
import EditorArea from './components/EditorArea';
import StatusBar from './components/StatusBar';
import { invoke } from '@tauri-apps/api/tauri';

const handleRegisterEventListener = async () => {
  const eventName = 'event-name';
  await invoke('register_event_listener', { event_name: eventName });
};

const { eventBus } = useNodiumApp();
eventBus.emit("event", "data");

// we can also listen to events from the tauri app
eventBus.on("event", (data) => {
    console.log(data);
});

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
    <Button onClick={handleRegisterEventListener}>Register Event Listener</Button>
        <SideBar />
        <EditorArea />
      </Box>
      <StatusBar />
    </Box>
  );
}

export default App;
