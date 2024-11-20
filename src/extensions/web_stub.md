# Safe Extensions
## Web Stub
> Crate features: [`web_stub`, `safe_extensions`]  
> Mutually exclusive with the `web` extension.

**This extensions is sandbox safe. It is enabled by default.**

Enables the following from javascript:

- `DOMException`
- `setImmediate`
- `setInterval`, and `clearInterval`
- `setTimeout`, and `clearTimeout`
- `atob` and `btoa`

### Usage Example:
```javascript
const base64 = btoa("Hello, world!");
setImmediate(() => {
  console.log(atob(base64));
});
```