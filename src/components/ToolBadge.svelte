<script lang="ts">
  let { tool } = $props<{ tool: string }>();

  function getColor(t: string) {
    const tLower = t.toLowerCase();
    if (tLower.includes('claude')) return 'bg-orange-500 text-white';
    if (tLower.includes('cursor')) return 'bg-blue-500 text-white';
    if (tLower.includes('windsurf')) return 'bg-teal-500 text-white';
    if (tLower.includes('codex')) return 'bg-purple-500 text-white';
    return 'bg-gray-500 text-white';
  }
</script>

<span class={`text-xs px-2 py-1 rounded-full font-medium ${getColor(tool)}`}>
  {tool}
</span>
