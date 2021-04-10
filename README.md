# Board Games
A full-stack rust implementation of Connect-4 and TOOT-and-OTTO.

### Client Side Installation
1. Navigate to the `client/` directory.
```sh
cd client/
```

2. Install `Trunk`, a WASM web application bundler for `Rust`. This may take a couple minutes.
```
cargo install trunk wasm-bindgen-cli
```

3. Run the application. This should automatically spawn a web browser. If this is not the case, open up a browser and navigate to `localhost:<PORT>` or `127.0.0.1:<PORT>`.
```sh
trunk serve # port 8080
# or specify the port number
trunk serve --port 3000
```
