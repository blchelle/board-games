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
1. Navigate to server/ directory
```sh
cd server/
```

2. Switch to `Rust Nightly`
```sh
rustup update
cargo update
rustup override set nightly
```

3. Setup the `MongoDB` server
```sh
mongod
```

4. In a new terminal, open the mongo shell
```sh
mongo
```

5. In the mongo shell, Initialize the users and scores collections with index for username on both of them
```sh
use 421ServerDB
db.createCollection("users")
db.users.createIndex({"username": 1}, {unique: true})
db.createCollection("scores")
db.scores.createIndex({"username": 1}, {unique: true})
```

6. Exit the mongo shell
```sh
exit
```

7. Run the `Rust Rocket` project
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

### `POST /update_score`

Update game stats

**JSON Request Format**
```json
{
	"username": "username",
	"game": 0, // Connect 4: 0, TootnOtto: 1
	"win": 0	// loss: 0, win: 1, tie: 2
}
```

### `GET /scores/<username>`

Gets game stats

**JSON Request Return Format**
```json
{
	"username": "username",
	"xo_wins": 0, // connect 4 wins
	"xo_loss": 0,	// connect 4 losses
	"xo_ties": 0,	// connect 4 ties
	"to_wins": 0,	// tootnotto wins
	"to_loss": 0, // tootnotto losses
	"to_ties": 0	// tootnotto ties
}
```
