# murmur3-wasm-bindgen

[wasm-bindgen](https://github.com/rustwasm/wasm-bindgen) wrapper for [`stusmall/murmur3`](https://github.com/stusmall/murmur3).

This is used for fuzz test against js implementation in [`@hiogawa/utils`](https://github.com/hi-ogawa/js-utils/blob/6f916a023d31f4269a51e68691e6799fac2768da/packages/utils/src/hash.ts#L45).

```sh
pnpm build
pnpm test
pnpm release
```
