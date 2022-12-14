# brainfuck-wasm

Brainfuck language interpreter in Rust, with VueJS frontend using WebAssembly.

## How to run

```
git clone https://github.com/JonPichel/brainfuck-wasm.git
cd brainfuck-wasm
npm install
npm run dev
```

## Recompiling the WASM module

```
cd brainfuck-rs
wasm-pack build
```
