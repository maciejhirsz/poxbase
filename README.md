# PoxBase

### Running the backend

You'll need a reasonably recent installation of Rust with `cargo`:

```
cd backend
cargo run --release
```

### Running the frontend

You'll need a reasonably recent installation of Node.js with `yarn`:

```
cd frontend
yarn
yarn start
```

### Production deployment

* To compile a binary of the backend without running it, run: `cargo build --release`. You can find the binary in `backend/target/release/poxbase`.
* To compile a production version of the frontend run `yarn build`. You will find the built files in `frontend/build`.

### License

PoxBase is free software, and is released under the terms of the GNU General Public
License version 3. See [LICENSE](LICENSE).
