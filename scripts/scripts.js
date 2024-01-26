const { detect } = require('esc-rs')
const fs = require('node:fs/promises')

const main = async () => {
  // const code = await fs.readFile(filename)
  const result = detect({
    browserslist: 'IE 11',
    filename: 'input.js',
    code: 'const a = 1 ?? 2',
    // code: code.toString('utf-8'),
  })
  console.log('js result', result)
}

main()
