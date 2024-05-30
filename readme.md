# HTTP server using crate warp

## GET route Returning a TEXT

Using dependencies tokio and warp:

```toml
[dependencies]
tokio = { version = "1.37.0", features = ["full"] }
warp = { version = "0.3.7", features = ["tls", "tokio-rustls"] }
```

Remember that you can add this by doing

```
cargo add tokio --features full
cargo add warp --features tls,tokio-rustls
```

Query with:

```
curl http://localhost:8000/api/name/peter
```

## GET route Returning a JSON body parsing into a struct

Using dependencies serde, serde_json, serde_derive and bytes:

```toml
[dependencies]
tokio = { version = "1.36.0", features = ["full"] }
warp = { version = "0.3.6", features = ["tls", "tokio-rustls"] }
serde = "1.0.203"
serde_json = "1.0.117"
serde_derive = "1.0.203"
```

Query with:

```
curl http://localhost:8000/api/person -d '{"name":"Peter", "age": 70}'
```
