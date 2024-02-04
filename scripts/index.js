import { readFile } from 'node:fs/promises'
import { join } from 'node:path'

import fg from 'fast-glob'

import { detect } from '../index'

const main = async () => {
  const files = fg.sync('**.js', {
    absolute: true,
    onlyFiles: true,
    cwd: join(process.cwd(), 'tests/fixtures'),
  })
  for (const filename of files) {
    const code = (await readFile(filename)).toString('utf-8')
    const result = detect({
      browserslist: 'Chrome > 68, IE 11, Edge > 17, and_qq > 11, > 0.5%, not dead',
      code,
      filename,
    })
    if (result.details.length) {
      console.log(filename)
      for (const detail of result.details) {
        console.log(`Reason: ${detail.feature}`, code.slice(detail.s, detail.e))
      }
    }
  }
}

main()
