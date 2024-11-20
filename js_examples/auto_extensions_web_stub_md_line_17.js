const base64 = btoa("Hello, world!");
setImmediate(() => {
  console.log(atob(base64));
});
