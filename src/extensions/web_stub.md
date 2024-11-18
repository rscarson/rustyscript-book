# Web Stub
> Crate features: [`web_stub`, `safe_extensions`]  

Mutually exclusive with the `web` extension.

Enables the following:

- DOMException
- setImmediate
- clearInterval
- clearTimeout
- setInterval
- setTimeout
- atob
- btoa

This extensions is sandbox safe. It is enabled by default.

-----

Usage Example:
```javascript
const base64 = btoa("Hello, world!");
setImmediate(() => {
  console.log(atob(base64));
});
```