# Board Games
A full-stack rust implementation of Connect-4 and TOOT-and-OTTO

### Client Side Installation
1. Navigate to the `client/` directory
```sh
cd client/
```

2. Compile the Rust project to Web Assembly
```sh
wasm-pack build --target web --out-name wasm --out-dir ./public/pkg
```

3. Navigate to the `client/public/` directory
```sh
cd public/ # assuming you're in client
```

4. Install the required npm dependencies
```sh
npm install
```

5. Start the development server
```sh
npm run dev
```

**Notes for Developers**
1. When modifying the `Rust` code, you will have to run `Step 2.` everytime a change is made
2. When modifying hte `SASS` stylesheets, you shouldn't have to restart the server, unless it crashes of course
