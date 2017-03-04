# emscripten-samples
Tiny Emscripten projects for showing some build configurations and features

Require the [uglifyjs](https://www.npmjs.com/package/uglify-js) cli, gzip, and the latest Emscripten tools. See [this repo](https://github.com/abdullahibnnadjo/emsdk-next/) for a light and simple installation of them.

The goal in each project is to find the right build options for generating the lightest 🍃 possible files (so good for the web 🌐) for different functionnalities and constraints, in asm.js and WebAssembly version.

Each project come in a C and a [Rust](https://www.rust-lang.org/en-US/) version, or a mixing of the two, if it's what we want to achieve.

You may require Chrome Canary or Firefox nightly if you want to test WebAssembly builds, and a simple [http-server](https://www.npmjs.com/package/http-server).

---

### Constraints :

- Provide a `dev` and `release` build
- Don't pollute the global scope (modularize)
