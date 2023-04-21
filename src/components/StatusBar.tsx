import React from "react";
import { Box, Button } from "@mui/material";

import { emit, listen } from "@tauri-apps/api/event";

const StatusBar: React.FC = () => {
  const handleButtonClick = () => {
    // Emit your event or perform any desired action here
    console.log("Button clicked");
    emit("event", {
      name: "reload",
      payload: "Hello from the frontend!",
    });
  };

  return (
    <Box
      sx={{
        height: "24px",
        backgroundColor: "#333",
        color: "white",
        display: "flex",
        alignItems: "center",
        paddingLeft: "16px",
      }}
    >
      {/* Add your status bar content here */}
      <Button onClick={handleButtonClick} variant="contained" size="small">
        Reload
      </Button>
    </Box>
  );
};

export default StatusBar;
