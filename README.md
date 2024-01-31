# esc-rs

# usage

```console
pnpm i esc-rs -D
```

Detect features with `browserlist`

```js
import { detect } from 'esc-rs'

const result = detect({
  filename: 'input.js',
  code: 'const a = 1 ?? false',
  browserslist: 'IE 11',
})

// result
// {
//   features: {
//     nullishCoalescing: true,
//     ...
//   }
// }
```

Will output `<feature>: true` if `<feature>` not support in current `browserslist`. 

> [!WARNING]  
> Currently unable to check polyfill features, e.g. `Async iterators`. In swc it will inject `core-js` polyfills instead of transform