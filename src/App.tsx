import React from "react";
import { Box } from "@mui/material";
import SideBar from "./components/SideBar";
import EditorArea from "./components/EditorArea";
import StatusBar from "./components/StatusBar";
import { AppProvider } from "./AppContext";

import "./App.css";
import "./styles/react-resizable-styles.css";

function App() {
  return (
    <AppProvider>
      <Box
        sx={{
          display: "flex",
          flexDirection: "column",
          height: "100vh",
          backgroundColor: "#1e1e1e",
          // no scroll no overflow
          overflow: "hidden",
          "&::-webkit-scrollbar": {
            display: "none",
          },
          "&::-webkit-scrollbar-track": {
            display: "none",
          },
        }}
      >
        <Box sx={{ flexGrow: 1, display: "flex" }}>
          <SideBar />
          <EditorArea />
        </Box>
        <StatusBar />
      </Box>
    </AppProvider>
  );
}

export default App;
