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
const single = async (filename: string, feature: string, shouldFound = true) => {
  const code = (await fs.readFile(filename)).toString('utf-8')
  const result = detect({
    filename,
    code,
    browserslist: 'IE 11',
  })
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
  const logicalAssignmentOperators = path.join(fixtures, './LogicalAssignmentOperators')
  it('logicalAssignmentOperators', async () => {
    const filename = path.join(logicalAssignmentOperators, './index.js')
    const code = (await fs.readFile(filename)).toString('utf-8')
    const result = detect({
      filename,
      code,
      browserslist: 'IE 11',
    })
    expect(result.features.logicalAssignmentOperators).toBe(true)
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
})