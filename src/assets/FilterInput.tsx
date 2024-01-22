const FilterInput = ({ filter, setFilter, className }: { filter: string; setFilter: (filter: string) => void; className?: string }) => {
  return (
    <label className={className}>
        <input
          type="text"
          className="row"
          value={filter}
          placeholder="Add filter..."
          onChange={(e) => setFilter(e.currentTarget.value)}
        />
    </label>
  );
};

export default FilterInput;
