# serde-gen

Generate rust structure from JSON value

# Tutorial

```sh
cargo build
curl -s 'https://en.wikipedia.org/w/api.php?action=query&list=recentchanges&rcprop=title%7Cids%7Csizes%7Cflags%7Cuser&format=json&rclimit=10' \
    | target/debug/serde_gen --in /dev/stdin --out /dev/stdout
```

# TODO

 - remove all `unimplemented!()` from codebase
 - extend to other serialization formats
