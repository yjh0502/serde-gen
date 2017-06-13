# serde-gen

Generate rust struct types from JSON value

# Tutorial

```sh
cargo build
curl -s 'https://en.wikipedia.org/w/api.php?action=query&list=recentchanges&rcprop=title%7Cids%7Csizes%7Cflags%7Cuser&format=json&rclimit=10' \
    | target/debug/serde_gen
```

# Help
```sh
target/debug/serde_gen --help
```

# TODO

 - remove all `unimplemented!()` from codebase
 - extend to other serialization formats
