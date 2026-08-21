<script lang="ts">
  import Sidebar from '../components/Sidebar.svelte';
  import SkillList from '../components/SkillList.svelte';
  import SkillEditor from '../components/SkillEditor.svelte';
  import { onMount } from 'svelte';
  import { tauriAPI } from '../lib/tauri';
  import { skillsState, collectionsState } from '../stores/app.svelte';

  onMount(async () => {
    try {
      skillsState.isLoading = true;
      // In a real app, we might need to handle errors or initial empty states gracefully
      // This is a placeholder since the rust backend commands might not be fully implemented yet
      try {
        const skills = await tauriAPI.scanAll();
        console.log("Loaded skills from Rust:", skills);
        skillsState.items = skills;
      } catch (e) {
        console.error("Failed to fetch skills (expected if Rust backend is WIP):", e);
      }

      try {
        const collections = await tauriAPI.getCollections();
        collectionsState.items = collections;
      } catch (e) {
        console.error("Failed to fetch collections:", e);
      }
      
    } finally {
      skillsState.isLoading = false;
    }
  });
</script>

<main class="flex h-screen w-full overflow-hidden bg-white dark:bg-gray-950 text-gray-900 dark:text-gray-100">
  <Sidebar />
  <SkillList />
  <SkillEditor />
</main>

