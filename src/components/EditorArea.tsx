import React from "react";
import { Box } from "@mui/material";
// import Draggable from "react-draggable";

const EditorArea: React.FC = () => {
  // Sample data for the grid
  const rows = [
    { id: 1, dot: "•" },
    { id: 2, dot: "•" },
    { id: 3, dot: "•" },
    // Add more rows as needed
  ];

  const columns = [
    { field: "id", headerName: "ID", width: 90 },
    { field: "dot", headerName: "Dot", width: 150 },
  ];

  return (
    <Box
      component="main"
      sx={{
        flexGrow: 1,
        backgroundColor: "#1e1e1e",
        color: "white",
        padding: "16px",
        overflowY: "auto",
      }}
    >
      {/* <Draggable></Draggable> */}
    </Box>
  );
};

export default EditorArea;
