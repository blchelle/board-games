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

### Server Side Installation
1. Switch to `Rust Nightly`
```sh
rustup update
cargo update
cd server/
rustup override set nightly
```

2. Setup the `MongoDB` server
```sh
# Insert some commands here
```

3. Initialize the `users` and `scores` collections with index for `username` on both of them
```sh
# Insert some commands here
```

4. Run the `Rust Rocket` project
```sh
cargo run
```

### Server Side API Routes

#### `GET /scores/{username}`

Returns the game scores of a user

#### `POST /login`

Login Request

**JSON Request Format**
```json
{
	"username": "username",
	"password": "password"
}
```

### `POST /new_user`

Sign up Request

**JSON Request Format**
```json
{
	"username": "username",
	"password": "password"
}
```