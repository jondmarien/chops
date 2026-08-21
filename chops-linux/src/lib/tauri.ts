import { invoke } from '@tauri-apps/api/core';
import type { Skill, Collection, SkillFilters, AppSettings } from './types';

export const tauriAPI = {
  // Scanner
  scanAll: () => invoke<Skill[]>('scan_all'),
  getSkills: (filters: SkillFilters) => invoke<Skill[]>('get_skills', { filters }),
  getSkill: (id: string) => invoke<Skill>('get_skill', { id }),

  // Editor
  saveSkill: (id: string, content: string, frontmatter: Record<string, string>) => 
    invoke<Skill>('save_skill', { id, content, frontmatter }),
  createSkill: (tool: string, name: string, content: string) => 
    invoke<Skill>('create_skill', { tool, name, content }),
  deleteSkill: (id: string) => invoke<void>('delete_skill', { id }),
  getSkillContent: (id: string) => invoke<string>('get_skill_content', { id }),

  // Collections
  createCollection: (name: string) => invoke<Collection>('create_collection', { name }),
  addSkillToCollection: (skillId: string, collectionId: string) => 
    invoke<void>('add_skill_to_collection', { skillId, collectionId }),
  removeSkillFromCollection: (skillId: string, collectionId: string) => 
    invoke<void>('remove_skill_from_collection', { skillId, collectionId }),
  getCollections: () => invoke<Collection[]>('get_collections'),

  // Settings
  addCustomScanPath: (path: string) => invoke<void>('add_custom_scan_path', { path }),
  removeCustomScanPath: (path: string) => invoke<void>('remove_custom_scan_path', { path }),
  getSettings: () => invoke<AppSettings>('get_settings'),
  setSetting: (key: string, value: any) => invoke<void>('set_setting', { key, value }),

  // File Watching
  startWatch: () => invoke<void>('start_watch'),
  stopWatch: () => invoke<void>('stop_watch'),
};
