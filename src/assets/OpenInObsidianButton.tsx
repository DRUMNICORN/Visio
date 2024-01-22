// OpenInObsidianButton will have obsidian icon and will open the current note in obsidian

import { SiObsidian } from 'react-icons/si';

const OpenInObsidianButton = ({ className, onClick }: { className?: string; onClick: () => void }) => {
  return (
    <button type="button" onClick={onClick} className={className} disabled={true}>
      Open Obsidian &nbsp; <SiObsidian />
    </button>
  );
};


export default OpenInObsidianButton;