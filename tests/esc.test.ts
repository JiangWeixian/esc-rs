import fs from 'node:fs/promises'
import path from 'node:path'

import fg from 'fast-glob'
import {
  describe,
  expect,
  it,
} from 'vitest'

import { detect } from '../index'

const fixtures = path.join(process.cwd(), './tests/fixtures')
const glob = async (cwd: string, feature: string, shouldFound = true) => {
  const files = fg.sync('**.js', {
    onlyFiles: true,
    cwd,
    absolute: true,
  })
  for (const filename of files) {
    const code = (await fs.readFile(filename)).toString('utf-8')
    const result = detect({
      filename,
      code,
      browserslist: 'IE 11',
    })
    expect(result.features[feature]).toBe(shouldFound)
  }
}
// eslint-disable-next-line unused-imports/no-unused-vars
const single = async (filename: string, feature: string, shouldFound = true) => {
  const code = (await fs.readFile(filename)).toString('utf-8')
  const result = detect({
    filename,
    code,
    browserslist: 'IE 11',
  })
  console.log(result)
  expect(result.features[feature]).toBe(shouldFound)
}
const spread = path.join(fixtures, './spread/should')

describe('es2022', () => {
  const classStaticBlock = path.join(fixtures, './ClassStaticBlock')
  const privateMethods = path.join(fixtures, './PrivateMethods')
  const classProperties = path.join(fixtures, './ClassProperties')
  it('classStaticBlock', async () => {
    const filename = path.join(classStaticBlock, './index.js')
    const code = (await fs.readFile(filename)).toString('utf-8')
    const result = detect({
      filename,
      code,
      browserslist: 'IE 11',
    })
    expect(result.features.classStaticBlock).toBe(true)
  })
  it('privateMethods', async () => {
    const filename = path.join(privateMethods, './index.js')
    const code = (await fs.readFile(filename)).toString('utf-8')
    const result = detect({
      filename,
      code,
      browserslist: 'IE 11',
    })
    expect(result.features.privateMethods).toBe(true)
  })
  it('classProperties', async () => {
    const filename = path.join(classProperties, './index.js')
    const code = (await fs.readFile(filename)).toString('utf-8')
    const result = detect({
      filename,
      code,
      browserslist: 'IE 11',
    })
    expect(result.features.classProperties).toBe(true)
  })
})

describe('es2021', () => {
  const yes = path.join(fixtures, './LogicalAssignmentOperators/should')
  it('logicalAssignmentOperators', async () => {
    await glob(yes, 'logicalAssignmentOperators')
  })
})

describe('es2020', () => {
  const nullishCoalescing = path.join(fixtures, './NullishCoalescing')
  const optionalChaining = path.join(fixtures, './OptionalChaining')
  it('nullishCoalescing', async () => {
    const filename = path.join(nullishCoalescing, './index.js')
    const code = (await fs.readFile(filename)).toString('utf-8')
    const result = detect({
      filename,
      code,
      browserslist: 'IE 11',
    })
    expect(result.features.nullishCoalescing).toBe(true)
  })

  it('optionalChaining', async () => {
    const filename = path.join(optionalChaining, './index.js')
    const code = (await fs.readFile(filename)).toString('utf-8')
    const result = detect({
      filename,
      code,
      browserslist: 'IE 11',
    })
    expect(result.features.optionalChaining).toBe(true)
  })
})

describe('es2019', () => {
  const optionalCatchBinding = path.join(fixtures, './OptionalCatchBinding')
  it('optionalCatchBinding', async () => {
    const filename = path.join(optionalCatchBinding, './index.js')
    const code = (await fs.readFile(filename)).toString('utf-8')
    const result = detect({
      filename,
      code,
      browserslist: 'IE 11',
    })
    expect(result.features.optionalCatchBinding).toBe(true)
  })
})

describe('es2018', () => {
  const objectRestSpread = path.join(fixtures, './ObjectRestSpread')
  describe('objectRestSpread', () => {
    it('should', async () => {
      await glob(objectRestSpread, 'objectRestSpread')
    })
    it('should not', async () => {
      await glob(spread, 'objectRestSpread', false)
    })
  })
})

describe('es2017', () => {
  const asyncToGenerator = path.join(fixtures, './AsyncToGenerator')
  it('asyncToGenerator', async () => {
    await glob(asyncToGenerator, 'asyncToGenerator')
  })
})

describe('es2016', () => {
  const exponentiationOperator = path.join(fixtures, './ExponentiationOperator')
  it('exponentiationOperator', async () => {
    await glob(exponentiationOperator, 'exponentiationOperator')
  })
})

describe('es2015', () => {
  const notSpread = path.join(fixtures, './spread/should-not')
  const destructuring = path.join(fixtures, './Destructuring/should')
  const notDestructuring = path.join(fixtures, './Destructuring/should-not')
  describe('spread', () => {
    it('should', async () => {
      await glob(spread, 'spread')
    })
    it('should not', async () => {
      await glob(notSpread, 'spread', false)
    })
  })
  describe('destructuring', () => {
    it('should', async () => {
      await glob(destructuring, 'destructuring')
    })
    it('should not', async () => {
      await glob(notDestructuring, 'destructuring', false)
    })
  })
  describe('parameters', () => {
    const yes = path.join(fixtures, './Parameters/should')
    const not = path.join(fixtures, './Parameters/should-not')
    it('should', async () => {
      await glob(yes, 'parameters')
    })
    it('should not', async () => {
      await glob(not, 'parameters', false)
    })
  })
  describe('arrowFunctions', () => {
    const yes = path.join(fixtures, './ArrowFunctions/should')
    it('should', async () => {
      await glob(yes, 'arrowFunctions')
    })
  })
  describe('blockScoping', () => {
    const yes = path.join(fixtures, './BlockScoping/should')
    it('should', async () => {
      await glob(yes, 'blockScoping')
    })
  })
  describe('templateLiterals', () => {
    const yes = path.join(fixtures, './TemplateLiterals/should')
    it('should', async () => {
      await glob(yes, 'templateLiterals')
    })
  })
  describe('shorthandProperties', () => {
    const yes = path.join(fixtures, './ShorthandProperties/should')
    it('should', async () => {
      await glob(yes, 'shorthandProperties')
    })
  })
  describe('computedProperties', () => {
    const yes = path.join(fixtures, './ComputedProperties/should')
    it('should', async () => {
      await glob(yes, 'computedProperties')
    })
  })
  describe('stickyRegex', () => {
    const yes = path.join(fixtures, './StickyRegex/should')
    it('should', async () => {
      await glob(yes, 'stickyRegex')
    })
  })
  describe('classes', () => {
    const yes = path.join(fixtures, './Classes/should')
    it('should', async () => {
      await glob(yes, 'classes')
    })
  })
  describe('for_of', () => {
    const yes = path.join(fixtures, './ForOf/should')
    it('should', async () => {
      await glob(yes, 'forOf')
    })
  })
  describe('typeof_symbol', () => {
    const yes = path.join(fixtures, './TypeOfSymbol/should')
    it('should', async () => {
      await glob(yes, 'typeofSymbol')
    })
  })
  describe('objectSuper', () => {
    const yes = path.join(fixtures, './ObjectSuper/should')
    const no = path.join(fixtures, './ObjectSuper/should-not')
    it('should', async () => {
      await glob(yes, 'objectSuper')
    })
    it('should-not', async () => {
      await glob(no, 'objectSuper', false)
    })
  })
  describe('newTarget', () => {
    const yes = path.join(fixtures, './NewTarget/should')
    const no = path.join(fixtures, './NewTarget/should-not')
    it('should', async () => {
      await glob(yes, 'newTarget')
    })
    it('should-not', async () => {
      await glob(no, 'newTarget', false)
    })
  })
  describe('functionName', () => {
    const yes = path.join(fixtures, './FunctionName/should')
    const no = path.join(fixtures, './FunctionName/should-not')
    it('should', async () => {
      await glob(yes, 'functionName')
    })
    it('should-not', async () => {
      await glob(no, 'functionName', false)
    })
  })
  describe('regenerator', () => {
    const yes = path.join(fixtures, './Regenerator/should')
    it('should', async () => {
      await glob(yes, 'regenerator')
    })
  })
})
