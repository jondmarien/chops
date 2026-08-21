<script lang="ts">
  import { appState, skillsState } from '../stores/app.svelte';
  import ToolBadge from './ToolBadge.svelte';
  import Fuse from 'fuse.js';

  let filteredSkills = $derived.by(() => {
    let result = skillsState.items;

    // Filter by collection
    if (appState.selectedCollection) {
      // Assuming skills have a way to know their collections (in actual app, this might be a join)
      // For MVP, just a stub logic or we handle it via IPC
    }

    // Filter by tools
    if (appState.toolFilters.length > 0) {
      result = result.filter(s => {
        // Handle both single tool_source and array tool_sources
        if (s.tool_sources) {
          return s.tool_sources.some(t => appState.toolFilters.includes(t));
        }
        return appState.toolFilters.includes(s.tool_source);
      });
    }

    // Search
    if (appState.searchText.trim() !== '') {
      const fuse = new Fuse(result, {
        keys: ['name', 'description', 'content'],
        threshold: 0.3,
      });
      result = fuse.search(appState.searchText).map(r => r.item);
    }

    // Sort
    result = [...result].sort((a, b) => a.name.localeCompare(b.name));

    return result;
  });

  function selectSkill(skill: any) {
    appState.selectedSkill = skill;
  }
</script>

<div class="flex-1 border-r border-gray-200 dark:border-gray-800 bg-white dark:bg-gray-950 flex flex-col h-full min-w-[300px] max-w-[400px]">
  <div class="p-4 border-b border-gray-200 dark:border-gray-800">
    <div class="relative">
      <div class="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
        <svg class="h-4 w-4 text-gray-400" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor">
          <path fill-rule="evenodd" d="M8 4a4 4 0 100 8 4 4 0 000-8zM2 8a6 6 0 1110.89 3.476l4.817 4.817a1 1 0 01-1.414 1.414l-4.816-4.816A6 6 0 012 8z" clip-rule="evenodd" />
        </svg>
      </div>
      <input 
        type="text" 
        bind:value={appState.searchText}
        class="block w-full pl-10 pr-3 py-2 border border-gray-300 dark:border-gray-700 rounded-md leading-5 bg-white dark:bg-gray-900 placeholder-gray-500 focus:outline-none focus:placeholder-gray-400 focus:border-blue-500 focus:ring-1 focus:ring-blue-500 sm:text-sm" 
        placeholder="Search skills..." 
      />
    </div>
  </div>

  <div class="flex-1 overflow-y-auto">
    {#if skillsState.isLoading}
      <div class="p-4 text-center text-gray-500">Loading skills...</div>
    {:else if filteredSkills.length === 0}
      <div class="p-8 text-center text-gray-500">
        <p class="mb-2">No skills found.</p>
        <p class="text-sm">Try adjusting your filters or search.</p>
      </div>
    {:else}
      <ul class="divide-y divide-gray-100 dark:divide-gray-800">
        {#each filteredSkills as skill (skill.id)}
          <li class="max-h-[120px] overflow-hidden">
            <button 
              class={`w-full h-full text-left p-4 hover:bg-gray-50 dark:hover:bg-gray-900 transition-colors ${appState.selectedSkill?.id === skill.id ? 'bg-blue-50 dark:bg-blue-900/20' : ''}`}
              onclick={() => selectSkill(skill)}
            >
              <div class="flex justify-between items-start mb-1">
                <h3 class="text-sm font-medium text-gray-900 dark:text-white truncate pr-2">{skill.name || "Untitled"}</h3>
                <ToolBadge tool={skill.tool_source} />
              </div>
              {#if skill.description}
                <p class="text-xs text-gray-500 dark:text-gray-400 line-clamp-2">{skill.description}</p>
              {/if}
            </button>
          </li>
        {/each}
      </ul>
    {/if}
  </div>
</div>
