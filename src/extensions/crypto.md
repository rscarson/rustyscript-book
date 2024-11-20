# Safe Extensions
## Crypto
> Crate features: [`crypto`, `safe_extensions`]  
> <https://crates.io/crates/deno_crypto/>  
> <https://www.w3.org/TR/WebCryptoAPI/>

Populates the global `CryptoKey`, `Crypto`, `crypto`, and `SubtleCrypto` objects
**This extensions is sandbox safe. It is enabled by default.**

### Options
**`RuntimeOptions::extension_options::crypto_seed`**
- Optional seed the deno_crypto RNG
- Default: `None`

If a seed is provided, then [`rand::rngs::StdRng`](https://docs.rs/rand/latest/rand/rngs/struct.StdRng.html) will be used to generate random numbers.  
Otherwise, [`rand::thread_rng`](https://docs.rs/rand/latest/rand/fn.thread_rng.html) will be used.

### Usage Example
```js
const key = await crypto.subtle.generateKey(
  {
    name: "AES-GCM",
    length: 256,
  },
  true,
  ["encrypt", "decrypt"],
);
```