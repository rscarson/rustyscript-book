# Crypto
> Crate features: [`crypto`, `safe_extensions`]  
> <https://crates.io/crates/deno_crypto/>  
> <https://www.w3.org/TR/WebCryptoAPI/>

Populates the global `CryptoKey`, `Crypto`, `crypto`, and `SubtleCrypto` objects

This extensions is sandbox safe. It is enabled by default.

-----

Usage Example:
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