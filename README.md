# esc-rs

> [!WARNING]  
> Based on swc which use [compat-data](https://github.com/babel/babel/blob/main/packages/babel-compat-data/data/plugins.json) from babel, maybe not as same as [caniuse](https://caniuse.com/), check this [issue](https://github.com/babel/babel/issues/16254#event-11678932441) for more details.

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

If want to report detail line-col info 

```tsx
const result = detect({
  filename: 'input.js',
  code: 'const a = 1 ?? false',
  browserslist: 'IE 11',
})
for (const detail of result.details) {
  console.log(`Reason: ${detail.feature}`, code.slice(detail.s, detail.e))
}
```

> [!WARNING]  
> Currently unable to check polyfill features, e.g. `Async iterators`. In swc it will inject `core-js` polyfills instead of transform