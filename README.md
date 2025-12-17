Experimentation with axum

This code is done to play with axum.

## quick start

### build and run

```bash
cargo run
```
By default, the webserver listen on `http://localhost:3000/shorten`

### store a new url

```bash
curl -d url=https://www.example.com/very/long/path http://localhost:3000/shorten
```

the answer contains the shortened url

### retrieve an url

```bash
curl http://localhost:3000/{id}
```
