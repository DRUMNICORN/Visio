import { Box, Drawer } from "@mui/material";
import { alpha } from "@mui/system";

interface SideBarWindowProps {
  content: React.ReactNode;
}

const SideBarWindow: React.FC<SideBarWindowProps> = ({ content }) => {
  return (
    <Box
      sx={{
        width: "100%",
        height: "100vh",
        overflowY: "auto",
        backgroundColor: alpha("#1E1E1E", 0.95),
        color: "white",
        fontFamily: "Consolas, Monaco, 'Andale Mono', 'Ubuntu Mono', monospace",
        fontSize: "14px",
        lineHeight: "20px",
        fontWeight: 400,
      }}
    >
      {content}
    </Box>
  );
};

export default SideBarWindow;
