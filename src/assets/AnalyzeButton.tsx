// OpenInObsidianButton will have obsidian icon and will open the current note in obsidian

import { TbAnalyze, TbAnalyzeFilled, TbAnalyzeOff } from 'react-icons/tb';

const AnalyzeButton = ({ className, onClick, disabled, loading, disabledText }: { className?: string; onClick: (e: any) => void, disabled?: boolean, loading?: boolean, disabledText?: boolean }) => {
  return (
    <button type="button" onClick={onClick} className={className} disabled={disabled}>
     {disabledText ? null : <span>  Analyze &nbsp; </span>}
     {loading ? <TbAnalyzeFilled className="spin" /> : disabled ? <TbAnalyzeOff /> : <TbAnalyze />}
    </button>
  );
};

export default AnalyzeButton;