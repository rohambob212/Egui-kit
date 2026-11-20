export type WidgetCategory = 'Basic' | 'Form' | 'Layout' | 'Advanced';

export type WidgetStatus = 'ready' | 'planned';

export type PartNumber = 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8;

export interface PropSpec {
  name: string;
  rustType: string;
  defaultValue: string;
  description: string;
}

export interface WidgetSpec {
  slug: string;
  name: string;
  gpuiName: string;
  category: WidgetCategory;
  part: PartNumber;
  status: WidgetStatus;
  summary: string;
  eguiSignature: string;
  gpuiMappingNote: string;
  usageRust: string;
  rustFilePath: string;
  props: PropSpec[];
}

export interface ThemePreset {
  id: string;
  name: string;
  mode: 'light' | 'dark';
  source: string;
  vars: Record<string, string>;
}

export interface CrateFileEntry {
  path: string;
  title: string;
  group: 'Workspace' | 'Core' | 'Components' | 'Examples';
  content: string;
}

export interface PartInfo {
  part: PartNumber;
  title: string;
  focus: string;
  status: 'completed' | 'scheduled';
}
