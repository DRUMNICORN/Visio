import './FilterList.css'; // Import your CSS file for styling

const FilterList = ({ filters, className, onFilterClick }: { filters: string[], className?: string, onFilterClick: (filter: string) => void }) => {
  return (
    <div className={`filter-list-container ${className}`}>
      <p>Filters:</p>
      <div className="filter-buttons-container">
        {filters.map((f, index) => (
          <button key={index} className="filter-button" onClick={() => onFilterClick(f)}>
            {f}
          </button>
        ))}
      </div>
    </div>
  );
};

export default FilterList;
