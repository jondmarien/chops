<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { EditorState } from '@codemirror/state';
  import { EditorView, lineNumbers, keymap } from '@codemirror/view';
  import { defaultKeymap, history, historyKeymap } from '@codemirror/commands';
  import { appState } from '../stores/app.svelte';
  import { tauriAPI } from '../lib/tauri';

  let editorContainer = $state<HTMLElement>();
  let view = $state<EditorView>();
  let isSaving = $state(false);

  // Re-create editor when selected skill changes
  $effect(() => {
    const skill = appState.selectedSkill;
    if (skill && editorContainer) {
      if (view) view.destroy();

      const startState = EditorState.create({
        doc: skill.content || '',
        extensions: [
          lineNumbers(),
          history(),
          keymap.of([...defaultKeymap, ...historyKeymap]),
          EditorView.theme({
            "&": { height: "100%", fontSize: "14px" },
            ".cm-scroller": { overflow: "auto" }
          }),
          EditorView.updateListener.of((update) => {
            if (update.docChanged) {
              // Mark as dirty or update local state
              // we won't mutate appState directly here to avoid infinite loops, but we could track dirty state.
            }
          })
        ]
      });

      view = new EditorView({
        state: startState,
        parent: editorContainer
      });
    } else if (!skill && view) {
      view.destroy();
      view = undefined as any;
    }
  });

  async function handleSave() {
    if (!appState.selectedSkill || !view) return;
    
    isSaving = true;
    try {
      const content = view.state.doc.toString();
      // Mocking frontmatter for now. Real implementation would parse frontmatter first.
      const frontmatter = appState.selectedSkill.frontmatter || {};
      
      const updatedSkill = await tauriAPI.saveSkill(appState.selectedSkill.id, content, frontmatter);
      appState.selectedSkill = updatedSkill;
      
      // Update the skill in the list
      // skillsState.items = skillsState.items.map(s => s.id === updatedSkill.id ? updatedSkill : s);
    } catch (e) {
      console.error("Failed to save:", e);
    } finally {
      isSaving = false;
    }
  }

  // Keyboard shortcut Ctrl+S
  function handleKeyDown(e: KeyboardEvent) {
    if ((e.ctrlKey || e.metaKey) && e.key === 's') {
      e.preventDefault();
      handleSave();
    }
  }

  onDestroy(() => {
    if (view) view.destroy();
  });
</script>

<svelte:window onkeydown={handleKeyDown} />

<div class="flex-1 flex flex-col h-full bg-white dark:bg-gray-950">
  {#if appState.selectedSkill}
    <div class="border-b border-gray-200 dark:border-gray-800 p-4 flex justify-between items-center bg-gray-50 dark:bg-gray-900">
      <div>
        <h2 class="text-lg font-semibold text-gray-900 dark:text-white">{appState.selectedSkill.name}</h2>
        <p class="text-sm text-gray-500">{appState.selectedSkill.file_path}</p>
      </div>
      <button 
        class="px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white rounded-md text-sm font-medium transition-colors disabled:opacity-50"
        onclick={handleSave}
        disabled={isSaving}
      >
        {isSaving ? 'Saving...' : 'Save'}
      </button>
    </div>
    
    <!-- CodeMirror Container -->
    <div bind:this={editorContainer} class="flex-1 overflow-hidden"></div>
  {:else}
    <div class="flex-1 flex items-center justify-center text-gray-500">
      <div class="text-center">
        <svg xmlns="http://www.w3.org/2000/svg" class="h-12 w-12 mx-auto text-gray-400 mb-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
        </svg>
        <p class="text-lg font-medium text-gray-900 dark:text-gray-300">No skill selected</p>
        <p class="text-sm mt-1">Select a skill from the list to view or edit it.</p>
      </div>
    </div>
  {/if}
</div>

<style>
  /* Base styles for CodeMirror to fit the container */
  :global(.cm-editor) {
    height: 100%;
  }
  :global(.cm-scroller) {
    font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, "Liberation Mono", "Courier New", monospace;
  }
</style>
