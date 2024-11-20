# IO Extensions
## FS
> Crate features: [`fs`, `io_extensions`]
> <https://crates.io/crates/deno_fs>  

Populates a large number of filesystem functions under `Deno.*`:  
**Not sandbox safe. Off by default**

**Full list of functions:**  
`writeFileSync` `writeFile` `writeTextFileSync` `writeTextFile` `readTextFile` `readTextFileSync` `readFile` `readFileSync` `watchFs` `chmodSync` `chmod` `chown` `chownSync` `copyFileSync` `cwd` `makeTempDirSync` `makeTempDir` `makeTempFileSync` `makeTempFile` `mkdirSync` `mkdir` `chdir` `copyFile` `readDirSync` `readDir` `readLinkSync` `readLink` `realPathSync` `realPath` `removeSync` `remove` `renameSync` `rename` `statSync` `lstatSync` `stat` `lstat` `truncateSync` `truncate` `FsFile` `open` `openSync` `create` `createSync` `symlink` `symlinkSync` `link` `linkSync` `utime` `utimeSync` `umask`

### Permissions
This extension is affected by the following methods in the permissions trait:
- `check_open` - Check if a given path is allowed to be opened
-----
- `check_read_all` - Can be used to disable all read operations
- `check_read` - Check if a given path is allowed to be read
- `check_read_blind` - `check_read`, but is expected to use the `display` argument to anonymize the path
-----
- `check_write_all` - Can be used to disable all write operations
- `check_write` - Check if a given path is allowed to be written to
- `check_write_blind` - `check_write`, but is expected to use the `display` argument to anonymize the path
- `check_write_partial` - Used to check if a given path is allowed to be written to partially (non-recursive removes, use this)

### Usage Example
```javascript
const file = Deno.openSync("/dev/zero");
const buf = new Uint8Array(100);
file.readSync(buf);

const file = Deno.openSync("/dev/null", { write: true });
file.writeSync(buf);
```