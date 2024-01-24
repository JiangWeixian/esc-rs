const { detect } = require('../index.js')


const result = detect({
  browserslist: 'Chrome > 63',
  'filename': 'input.js',
  code: 'const a = 1 ?? 2'
})

console.log('js result', result)