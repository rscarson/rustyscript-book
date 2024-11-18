# NodeJS Compatibility
With the `node_experimental` crate-level feature, you can enable support for some NodeJS APIs. It will also enable all other extension features.  
> Please note that this API is highly experimental, and likely does not support all node modules.
> Kindly report any issues you encounter.

## Usage
To enable the feature, add `node_experimental` to the `features` list in your `Cargo.toml`.

Node modules will be located in the `node_modules` directory using the `package.json` file in the current working directory.

You can import from the node standard library (Deno polyfills)
- For example `import os from "node:os"`

Or from the `node_modules` directory
- For example `import chalk from "npm:chalk@5";`

-----

See [this example](https://github.com/rscarson/rustyscript/tree/master/examples/node_import) for more information.