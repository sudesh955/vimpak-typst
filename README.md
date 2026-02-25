# @vimpak/typst

WASM bindings for [Typst](https://typst.app/), a modern typesetting system.

## Features

- **Compile**: Convert Typst markup to SVG output
- **Format**: Format Typst source code using typstyle

## Usage

### JavaScript/TypeScript

```javascript
import init, { compile, format } from "./pkg/vpk_typst.js";

await init();

const source = `
= Hello World
This is a test document.
`;

const svg = compile(source);
console.log(svg);

const formatted = format(source);
console.log(formatted);
```

## Building

```bash
wasm-pack build --scope vimpak --target web
```

## License

MIT or Apache-2.0
