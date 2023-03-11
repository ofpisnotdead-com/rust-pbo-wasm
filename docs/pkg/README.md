# OFP PBO Rust/WASM reader

See `src/lib-original.rs` for Rust PBO entries reader and `src/lib.rs` with added WASM glue.

Locally just run `build.sh`, and run some static server to serve `docs`.

For example with Ruby/Python built-in server you can do following.

```bash
./build.sh
ruby -run -e httpd docs/ # if you have Ruby installed
python -m http.server --directory docs 8080 # if you have Python 3 installed
# now open localhost:8080 in browser
```
