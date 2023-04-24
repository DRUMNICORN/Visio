// ResizableDrawer.tsx
import React from "react";
import { Drawer } from "@mui/material";
import { ResizableBox } from "react-resizable";

import "../styles/react-resizable-styles.css";

interface ResizableDrawerProps {
  open: boolean;
  onClose: () => void;
  drawerWidth: number;
  onResize: (event: React.SyntheticEvent, data: { size: { width: number; height: number } }) => void;
  children: React.ReactNode;
}

const CustomResizeHandle = React.forwardRef<HTMLDivElement>((_, ref) => (
  <div
    ref={ref}
    style={{
      cursor: "ew-resize",
      position: "absolute",
      top: 0,
      right: -5,
      bottom: 0,
      width: 10,
      zIndex: 2,
    }}
  />
));

const ResizableDrawer: React.FC<ResizableDrawerProps> = ({ open, onClose, drawerWidth, onResize, children }) => {
  return (
    <ResizableBox
      width={drawerWidth}
      onResize={onResize}
      axis="x"
      resizeHandles={["e"]}
      style={{
        height: "100vh",
      }}
      handle={(handleAxis, ref) => <CustomResizeHandle key={handleAxis} ref={ref} />}
    >
      <Drawer
        anchor="left"
        open={open}
        onClose={onClose}
        variant="persistent"
        style={{
          position: "absolute",
          top: 0,
          left: 0,
          width: "100%",
          height: "100%",
          backgroundColor: "rgba(0, 0, 0, 0.1)",
        }}
        PaperProps={{
          style: {
            backgroundColor: "rgba(0, 0, 0, 0)",
            color: "white",
            position: "relative",
            zIndex: 1,
            width: "100%",
            height: "100%",
            userSelect: "none",
          },
        }}
      >
        {children}
      </Drawer>
    </ResizableBox>
  );
};

export default ResizableDrawer;
