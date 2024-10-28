import { join } from 'node:path'

import { lookup } from '../index'

const dirname = 'unminify'

const l = async () => {
  const root = join(process.cwd(), 'tests/fixtures')
  const filename = join(root, `source-maps/${dirname}/server-entry.js.map`)
  const info = lookup({
    filename,
    details: [
      {
        feature: '',
        s: 0,
        e: 0,
        ls: { l: 436132, c: 11 },
        le: { l: 436132, c: 21 },
      },
    ],
  })
  console.log(info)
}

l()
