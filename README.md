## Rust Playground

### Setup
<b>Install Rust if you haven't done so</b>
<br>
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
<br>
<b>Install wasm-pack</b>
<br>

```bash
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
```
<br>
<b>Install http to serve the web file</b>
<br>

```bash
cargo install https
```

### Run the code (in ./rpg)
<b>Compiling the code</b>
<br>

```bash
wasm-pack build --target web --dev
```
<br>
<b>Serve up the content</b>
<br>

```bash
http
```
