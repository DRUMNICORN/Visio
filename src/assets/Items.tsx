
import './Items.css'; // Import the CSS file
import ItemsContainer from './ItemsContainer';

interface ItemsProps {
  paths: string[];
  filters: string[];
  onToggleFolder: (path: string, isOpen: boolean) => void;
}

const Items = ({ paths, filters, onToggleFolder }: ItemsProps) => {
  return <ItemsContainer paths={paths} filters={filters} onToggleFolder={onToggleFolder} />;
};

export default Items;

