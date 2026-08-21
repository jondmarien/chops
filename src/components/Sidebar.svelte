<script lang="ts">
  import { appState, collectionsState } from '../stores/app.svelte';

  const tools = [
    { label: 'Claude Code', value: 'Claude Code' },
    { label: 'Cursor', value: 'Cursor' },
    { label: 'Windsurf', value: 'Windsurf' },
    { label: 'Codex', value: 'Codex' },
    { label: 'Amp', value: 'Amp' },
    { label: 'Global', value: 'Global (Agents)' }
  ];

  function toggleTool(toolValue: string) {
    if (appState.toolFilters.includes(toolValue)) {
      appState.toolFilters = appState.toolFilters.filter(t => t !== toolValue);
    } else {
      appState.toolFilters = [...appState.toolFilters, toolValue];
    }
  }

  function selectCollection(collection: any | null) {
    appState.selectedCollection = collection;
  }
</script>

<aside class="w-64 border-r border-gray-200 dark:border-gray-800 bg-gray-50 dark:bg-gray-900 h-full flex flex-col p-4 overflow-y-auto">
  <div class="mb-8">
    <h2 class="text-xs font-semibold text-gray-500 uppercase tracking-wider mb-3">Library</h2>
    <ul class="space-y-1">
      <li>
        <button 
          class={`w-full text-left px-2 py-1.5 rounded-md text-sm ${!appState.selectedCollection ? 'bg-blue-100 dark:bg-blue-900/30 text-blue-700 dark:text-blue-300' : 'hover:bg-gray-200 dark:hover:bg-gray-800'}`}
          onclick={() => selectCollection(null)}
        >
          All Skills
        </button>
      </li>
    </ul>
  </div>

  <div class="mb-8">
    <div class="flex items-center justify-between mb-3">
      <h2 class="text-xs font-semibold text-gray-500 uppercase tracking-wider">Collections</h2>
      <button class="text-gray-400 hover:text-gray-600 dark:hover:text-gray-300" aria-label="Toggle collections">
        <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4"/></svg>
      </button>
    </div>
    <ul class="space-y-1">
      {#each collectionsState.items as collection}
        <li>
          <button 
            class={`w-full text-left px-2 py-1.5 rounded-md text-sm ${appState.selectedCollection?.id === collection.id ? 'bg-blue-100 dark:bg-blue-900/30 text-blue-700 dark:text-blue-300' : 'hover:bg-gray-200 dark:hover:bg-gray-800'}`}
            onclick={() => selectCollection(collection)}
          >
            {collection.name}
          </button>
        </li>
      {/each}
      {#if collectionsState.items.length === 0}
        <li class="text-sm text-gray-400 px-2 italic">No collections yet</li>
      {/if}
    </ul>
  </div>

  <div>
    <h2 class="text-xs font-semibold text-gray-500 uppercase tracking-wider mb-3">Tools</h2>
    <ul class="space-y-1">
      {#each tools as tool}
        <li>
          <label class="flex items-center px-2 py-1.5 rounded-md hover:bg-gray-200 dark:hover:bg-gray-800 cursor-pointer">
            <input 
              type="checkbox" 
              class="rounded border-gray-300 text-blue-600 focus:ring-blue-500 mr-2"
              checked={appState.toolFilters.includes(tool.value)}
              onchange={() => toggleTool(tool.value)}
            />
            <span class="text-sm">{tool.label}</span>
          </label>
        </li>
      {/each}
    </ul>
  </div>
</aside>
