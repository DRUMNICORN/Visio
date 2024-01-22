// ItemsTable.tsx
import React from 'react';

interface ItemsTableProps {
  headers: string[];
  rows: React.ReactNode[];
}

const ItemsTable: React.FC<ItemsTableProps> = ({ headers, rows }) => {
  return (
    <table className="items-table">
      <thead>
        <tr>
          {headers.map((header, index) => (
            <th key={index}></th>
          ))}
        </tr>
      </thead>
      <tbody>{rows}</tbody>
    </table>
  );
};

export default ItemsTable;
