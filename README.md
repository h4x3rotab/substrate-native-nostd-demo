# native-nostd-demo

This repo shows why "native-nostd" is necessary for substrate runtime ([#5547](https://github.com/paritytech/substrate/issues/5547)).
It basically invokes `Header::hash()` in a `no_std` environment. Because substrate runtime interface
assume a wasm environment when it's `no_std`, calling `Header::hash()` will result in a wasm
interface call.

Package `notcompile` shows exact the above problem. The compiler complains:

> undefined reference to `ext_hashing_blake2_256_version_1'

In package `compile`, I replaced the dependencies by [my fix](https://github.com/paritytech/substrate/pull/5743).

You can check the diff by running `./diff.sh`. The only diff is the `Cargo.toml` file:

```diff
12,13c12,13
< git = "https://github.com/paritytech/substrate"
< rev = "8a47b4ff049409b0d5d89abaf63f42dd9f3ab9f0"
---
> git = "https://github.com/h4x3rotab/substrate"
> branch = "native-nostd"
17,18c17,18
< git = "https://github.com/paritytech/substrate"
< rev = "8a47b4ff049409b0d5d89abaf63f42dd9f3ab9f0"
---
> git = "https://github.com/h4x3rotab/substrate"
> branch = "native-nostd"
22,23c22,23
< git = "https://github.com/paritytech/substrate"
< rev = "8a47b4ff049409b0d5d89abaf63f42dd9f3ab9f0"
---
> git = "https://github.com/h4x3rotab/substrate"
> branch = "native-nostd"
43a44
>   "sp-io/native-nostd",  # enable native-nostd for sp-io::hashing
```