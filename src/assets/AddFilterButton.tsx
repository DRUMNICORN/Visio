import { IoMdAdd } from 'react-icons/io';

const AddFilterButton = ( { className, onClick, disabled }: { className?: string; onClick: () => void, disabled?: boolean }) => {
  return (
    <button type="button" onClick={onClick} className={className} disabled={disabled}>
      <IoMdAdd />
    </button>
  );
};

export default AddFilterButton;
