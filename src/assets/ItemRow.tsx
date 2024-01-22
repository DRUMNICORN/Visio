// ItemRow.tsx
import React, { useEffect } from "react";
import AnalyzeButton from "./AnalyzeButton";
import { invoke } from "@tauri-apps/api/tauri";

interface ItemRowProps {
  isFolder: boolean;
  isOpen: boolean;

  path: string;
  name: string;
  type: string;
  // count: number;
  filter: string[];

  onClick: () => void;
}

const ItemRow: React.FC<ItemRowProps> = ({ isFolder, isOpen, path, name, type, filter, onClick }) => {
  const [count, setCount] = React.useState<number>(0);
  const [isLoading, setIsLoading] = React.useState<boolean>(false);

  const handleAnalyzeClick = async (e: React.MouseEvent<HTMLButtonElement, MouseEvent>) => {
    e.stopPropagation();

    if (!isFolder) {
      // Check for markdown file
      const hasMarkdown = await checkMarkdown();

      if (hasMarkdown) {
        // Reload the markdown file (add your logic here)
        reloadMarkdown();
      }
    }
  };

  const checkMarkdown = async () => {
    // Implement your logic to check for markdown file
    // For example, you can use Tauri invoke to check for the file
    const hasMarkdown = await invoke("check_markdown", { path: path });
    return hasMarkdown;
  }

  const reloadMarkdown = async () => {
    // Implement your logic to reload the markdown file
    // For example, you can use Tauri invoke to trigger a reload
    await invoke("reload_markdown", { path: path });
    console.log(`Markdown file ${path} reloaded`);
  };

  const getTotalCount = async (): Promise<number> => {
    let totalCount = 0;
    const total_count = await invoke<number>("get_total_count", { path: path, filter: filter});
    totalCount = total_count as number;
    return totalCount;
  };
  
  useEffect(() => {
    if (isFolder) {
      const fetchTotalCount = async () => {
        const totalCount = await getTotalCount() || 0;
        setCount(totalCount as number);
        setIsLoading(false);
      };
      
      setIsLoading(true);
      fetchTotalCount();
    }
  }, [isFolder, isOpen, name]);

  return (
    <tr>
      <td style={{ textAlign: "left" }}>
        {isFolder ? (
          <button className={""} onClick={onClick}>
            <AnalyzeButton className="small" disabled={isFolder || isLoading} disabledText={true} onClick={handleAnalyzeClick} />  &nbsp;
            {name}
          </button>
        ) : (
          <button className={"small"} onClick={() => {}}>
            <AnalyzeButton className="small" disabled={isFolder || isLoading} disabledText={true} onClick={handleAnalyzeClick} />  &nbsp;
            {name}
          </button>
        )}
      </td>
      <td>{type}</td>
      <td>{isFolder ? `${count} items` : ''}</td>
    </tr>
  );
};

export default ItemRow;
