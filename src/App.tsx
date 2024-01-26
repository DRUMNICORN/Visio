import {  useState } from 'react';
import reactLogo from './assets/react.svg';
import { listen } from '@tauri-apps/api/event';
import { invoke } from '@tauri-apps/api/tauri';
import './App.css';
import { SiObsidian } from 'react-icons/si';
import { IoIosFolder } from 'react-icons/io';

import Items from './assets/Items';
import FilterInput from './assets/FilterInput';
import FilterList from './assets/FilterList';
import AddFilterButton from './assets/AddFilterButton';
import OpenInObsidianButton from './assets/OpenInObsidianButton';
import AnalyzeButton from './assets/AnalyzeButton';

function App() {
  const [name, setName] = useState('');
  const [isLoading, setIsLoading] = useState(false);
  const [items, setItems] = useState<string[]>([]);
  const [filter, setFilter] = useState('');
  const [filters, setFilters] = useState<string[]>([]);
  const [isLoadButtonDisabled, setIsLoadButtonDisabled] = useState(true);

  // Listen for the folder-selected event
  listen('folder-selected', (event: any) => {
    if (event.payload == "null") return;

    setName(event.payload.replace(/\"/gm, '') as string);
    setIsLoadButtonDisabled(false); // Enable the "Load Folder" button when a folder is selected
  });

  // function handle folder select
  async function handleFolderSelect(): Promise<void> {
    console.log('handleFolderSelect');
    const result = await invoke<{ canceled: boolean; path?: string }>(
      'open_folder_dialog',
      {
        title: 'Select a folder',
        directory: true,
      }
    );

      console.log(result);

    if (!result.canceled && result.path) {
      if (result.path == "null") return;
      setName(result.path.replace(/\"/gm, '') as string);
      setIsLoadButtonDisabled(false); // Enable the "Load Folder" button when a folder is selected
    }
  }

  function handleRemoveFilter(filter: string): void {
    setFilters(filters.filter((f) => f !== filter));
  }

  // function to add filter
  function handleAddFilter(): void {
    if (filter.trim() !== '' && !filters.includes(filter)) {
      setFilters([...filters, filter.trim()]);
      setFilter('');
    }
  }

  // function handle folder load
  async function handleFolderLoad(): Promise<void> {
    setIsLoading(true);
    await invoke<string[]>('init_vault', {
      path: name,
    });

    const _items = await invoke<string[]>('load_folder', {
      path: name,
    });
    setItems(_items);

    // load_gitignore
    let git_ignore_path = name + '/.gitignore';
    const _gitignore = await invoke<string[]>('load_gitignore', {
      path: git_ignore_path,
    });
    setFilters(_gitignore);
    // setIsLoading(false);
  }

  return (
    <div className="container">
      <h1>Welcome to Nodium!</h1>

      <div className="row main">
        <a href="https://vitejs.dev" target="_blank">
          <img src="/vite.svg" className="logo vite" alt="Vite logo" />
        </a>
        <a href="https://tauri.app" target="_blank">
          <img src="/tauri.svg" className="logo tauri" alt="Tauri logo" />
        </a>
        <a href="https://reactjs.org" target="_blank">
          <img src={reactLogo} className="logo react" alt="React logo" />
        </a>
        <a href="https://obsdian.md" target="_blank">
          <SiObsidian className="logo obsidian" size={100} />
        </a>
      </div>

      <div className="container">
        <form
          className="row"
          onSubmit={(e) => {
            e.preventDefault();
          }}
        >
          <input
            id="greet-input"
            style={{ width: '30rem' }}
            onChange={(e) => setName(e.currentTarget.value)}
            value={name}
            placeholder="Select Any Folder..."
          />
          {/* :button for select folder dialog */}
          <button type="button" onClick={handleFolderSelect}>
            {/* select folder icon */}
            <IoIosFolder size={20} />
          </button>
        </form>
        <form
          className="row"
          onSubmit={(e) => {
            e.preventDefault();
          }}>
          <div className="container">
            <div className="row">
              <AnalyzeButton className="row" onClick={handleFolderLoad} disabled={isLoadButtonDisabled} loading={isLoading} />
              <OpenInObsidianButton className="row" onClick={() => invoke('open_in_obsidian', { path: name })} />
              <FilterInput className="row" filter={filter} setFilter={setFilter} />
              <AddFilterButton className="row" onClick={handleAddFilter} disabled={filter.trim() === ''} />
            </div>
            {filters.length > 0 && <FilterList className="row" filters={filters} onFilterClick={handleRemoveFilter} />}
          </div>
        </form>


        <div className="row">
          {items.length > 0 && <Items paths={items} filters={filters} onToggleFolder={() => { }} />}
        </div>
      </div>
    </div>
  );
}
export default App;
