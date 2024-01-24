const { detect } = require('../index.js')
const fs = require('node:fs/promises')

const main = async () => {
  const code = await fs.readFile(filename)
  const result = detect({
    browserslist: 'Chrome > 63',
    filename,
    code: code.toString('utf-8'),
  })
  console.log('js result', result)
}

main()
