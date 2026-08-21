import type { Skill, Collection } from '../lib/types';

export const appState = $state({
  selectedSkill: null as Skill | null,
  selectedCollection: null as Collection | null,
  searchText: '',
  toolFilters: [] as string[],
  sortBy: 'name',
  isDarkMode: false,
});

export const skillsState = $state({
  items: [] as Skill[],
  isLoading: false,
  error: null as string | null,
});

export const collectionsState = $state({
  items: [] as Collection[],
  isLoading: false,
  error: null as string | null,
});
