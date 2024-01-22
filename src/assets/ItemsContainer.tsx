import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import React from 'react';
import ItemRow from './ItemRow';
import ItemsTable from './ItemsTable';

interface ItemsContainerProps {
    paths: string[];
    filters: string[];
    onToggleFolder: (path: string, isOpen: boolean) => void;
}

const ItemsContainer: React.FC<ItemsContainerProps> = ({ paths, filters, onToggleFolder }) => {
    const [itemDetails, setItemDetails] = useState<{ [path: string]: number }>({});
    const [openFolders, setOpenFolders] = useState<{ [path: string]: boolean }>({});
    const [folderChildren, setFolderChildren] = useState<{ [path: string]: string[] }>({});

    async function getFolderItems(path: string): Promise<string[]> {
        const items = await invoke<string[]>('load_folder', {
            path: path,
        });

        return items;
    }

    const handleFolderClick = async (path: string) => {
        setOpenFolders((prevOpenFolders) => ({
            ...prevOpenFolders,
            [path]: !prevOpenFolders[path],
        }));

        if (!folderChildren[path]) {
            const children = await getFolderItems(path);
            setFolderChildren((prevFolderChildren) => ({
                ...prevFolderChildren,
                [path]: children,
            }));
        }

        onToggleFolder(path, !openFolders[path]);
    };

    useEffect(() => {
        const fetchItemDetails = async () => {
            const details: { [path: string]: number } = {};

            for (const path of paths) {
                const count = await getItemCount(path);
                details[path] = count;
            }

            setItemDetails(details);
        };

        fetchItemDetails();
    }, [paths]);

    async function getItemCount(path: string): Promise<number> {
        const count = await invoke<number>('get_item_count', {
            path: path,
            filters: filters,
        });

        return count;
    }

    function gitignoreToRegexp(pattern: string): RegExp {
        let regexStr = '';

        for (let char of pattern) {
            switch (char) {
                case '/':
                    regexStr += '\\/';
                    break;
                case '*':
                    regexStr += '.*';
                    break;
                case '?':
                    regexStr += '.';
                    break;
                case '[':
                    regexStr += '\\[';
                    break;
                case ']':
                    regexStr += '\\]';
                    break;
                case '{':
                    regexStr += '\\{';
                    break;
                case '}':
                    regexStr += '\\}';
                    break;
                case '(':
                    regexStr += '\\(';
                    break;
                case ')':
                    regexStr += '\\)';
                    break;
                case '+':
                    regexStr += '\\+';
                    break;
                case '^':
                    regexStr += '\\^';
                    break;
                case '$':
                    regexStr += '\\$';
                    break;
                case '.':
                    regexStr += '\\.';
                    break;
                case '|':
                    regexStr += '\\|';
                    break;
                default:
                    regexStr += char;
            }
        }

        return new RegExp(regexStr);
    }

    const isFiltered = (path: string): boolean => {
        // Check if path matches any filter regex
        const filterResult = filters.some(filterRegex => gitignoreToRegexp(filterRegex).test(path + '/'));

        // Check if path includes default paths
        const defaultPathResult = ['obsidian', 'docs', '.gitignore', '.git'].some(defaultPath => path.includes(defaultPath));

        // Final result
        return filterResult || defaultPathResult;
    };

    // Sort folders and files separately
    const sortedPaths = paths
        .filter((path) => !isFiltered(path))
        .sort((a, b) => {
            const isFolderA = itemDetails[a] !== -1;
            const isFolderB = itemDetails[b] !== -1;

            if (!isFolderA && !isFolderB) {
                // Sort folders based on their count (large to small)
                return itemDetails[b] - itemDetails[a];
            } else if (isFolderA && isFolderB) {
                // Sort files based on their count (small to large)
                return itemDetails[a] - itemDetails[b];
            } else {
                // Folders come before files
                return isFolderB ? 1 : -1;
            }
        });

    const items = sortedPaths.map((path, index) => {
        const isOpen = openFolders[path] || false;
        const children = folderChildren[path] || [];
        const isFolder = itemDetails[path] !== -1;

        return (
            <React.Fragment key={index}>
                <ItemRow
                    isFolder={isFolder}
                    name={path.split('/').pop() || ''}
                    type={isFolder ? 'Folder' : 'File'}
                    // count={isFolder ? itemDetails[path] || 0 : 0}
                    path={path}
                    filter={filters}
                    isOpen={isOpen}
                    onClick={() => handleFolderClick(path)}
                />
                {isFolder && isOpen && (
                    <tr>
                        <td colSpan={3} style={{ paddingLeft: '2rem', borderLeft: '1px solid #ccc' }}>
                            <ItemsContainer paths={children} filters={filters} onToggleFolder={onToggleFolder} />
                        </td>
                    </tr>
                )}
            </React.Fragment>
        );
    });

    const headers = [(paths[0] || '').split('/').splice(-2, 1)[0] || 'Name', '', ''];

    return <ItemsTable headers={headers} rows={items} />;
};

export default ItemsContainer;
