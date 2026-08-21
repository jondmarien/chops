export interface Skill {
  id: string;
  file_path: string;
  resolved_path: string;
  tool_source: string;
  is_directory: boolean;
  is_global: boolean;
  name: string;
  description?: string;
  content?: string;
  frontmatter?: Record<string, any>;
  installed_paths?: string[];
  tool_sources?: string[];
  file_modified_date?: string;
  file_size?: number;
  item_kind?: 'skill' | 'agent' | 'rule';
  created_at?: string;
  updated_at?: string;
}

export interface Collection {
  id: string;
  name: string;
  created_at: string;
  updated_at: string;
}

export interface SkillFilters {
  tools?: string[];
  kinds?: string[];
  search?: string;
  collection_id?: string;
}

export interface AppSettings {
  custom_scan_paths: string[];
  theme: 'light' | 'dark' | 'system';
}
