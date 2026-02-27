export type ParamType =
  | { type: "number"; min?: number; max?: number }
  | { type: "string" }
  | { type: "bool" }
  | { type: "trackIndex" }
  | { type: "clipIndex" }
  | { type: "filePath" };

export interface SkillParam {
  name: string;
  paramType: ParamType;
  description: string;
  required: boolean;
  defaultValue: unknown;
}

export type SkillCategory =
  | "Transport"
  | "Track"
  | "Clip"
  | "Search"
  | "Utility";

export interface SkillDescriptor {
  id: string;
  name: string;
  description: string;
  category: SkillCategory;
  params: SkillParam[];
  keyboardShortcut: string | null;
}

export interface SkillResult {
  success: boolean;
  message: string;
}
