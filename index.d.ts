/* tslint:disable */
/* eslint-disable */

/* auto-generated by NAPI-RS */

export interface FeaturesFlag {
  nullishCoalescing: boolean
  objectRestSpread: boolean
  optionalChaining: boolean
  optionalCatchBinding: boolean
}
export interface ParseOptions {
  target?: string
  browserslist: string
  filename: string
  code: string
}
export interface DetectResult {
  features: FeaturesFlag
  esVersions: Record<string, boolean>
}
export function detect(options: ParseOptions): DetectResult