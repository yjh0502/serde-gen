# serde-gen

Generate rust struct types from JSON value

[![Build Status](https://travis-ci.org/yjh0502/serde-gen.svg?branch=master)](https://travis-ci.org/yjh0502/serde-gen)

# Tutorial

```sh
cargo build --examples
curl -s 'https://en.wikipedia.org/w/api.php?action=query&list=recentchanges&rcprop=title%7Cids%7Csizes%7Cflags%7Cuser&format=json&rclimit=10' \
    | target/debug/serde_gen
```

Or, generate with [serde-gen-workers](https://github.com/yjh0502/serde-gen-workers/tree/master/frontend)
```sh
curl -s 'https://en.wikipedia.org/w/api.php?action=query&list=recentchanges&rcprop=title%7Cids%7Csizes%7Cflags%7Cuser&format=json&rclimit=10' \
    | curl -XPOST --data '@-' 'https://rustgen.jyu.workers.dev/schema'
```

# Help
```sh
target/debug/serde_gen --help
```

# TODO

 - remove all `unimplemented!()` from codebase
 - extend to other serialization formats
