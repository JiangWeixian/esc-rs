/* tslint:disable */
/* eslint-disable */

/* auto-generated by NAPI-RS */

export interface FeaturesFlag {
  spread: boolean
  classProperties: boolean
  destructuring: boolean
  computedProperties: boolean
  shorthandProperties: boolean
  stickyRegex: boolean
  templateLiterals: boolean
  parameters: boolean
  arrowFunctions: boolean
  blockScoping: boolean
  exponentiationOperator: boolean
  classStaticBlock: boolean
  privateMethods: boolean
  asyncToGenerator: boolean
  logicalAssignmentOperators: boolean
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
