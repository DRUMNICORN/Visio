// ItemsList.tsx
import React from 'react';

interface ItemsListProps {
  items: React.ReactNode[];
}

const ItemsList: React.FC<ItemsListProps> = ({ items }) => {
  return <>{items}</>;
};

export default ItemsList;
