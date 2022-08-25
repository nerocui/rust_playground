## Rust Playground

### Setup
<b>Install Rust if you haven't done so</b>
`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
<b>Install wasm-pack</b>
`curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh`
<b>Install http to serve the web file</b>
`cargo install https`

### Run the code (in ./rpg)
<b>Compiling the code</b>
`wasm-pack build --target web --dev`
<b>Serve up the content</b>
`http`
