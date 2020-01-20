# iceoryx-rs

Experimental rust wrapper for the [iceoryx](https://github.com/eclipse/iceoryx) IPC middleware.

Disclaimer: This is a personal side project and not related to my employer!

# clone an build

The iceoryx repo is include as git submodule, therefore keep in mind to checkout with the `--recursive` option.
```
git clone https://github.com/elBoberido/iceoryx-rs.git --recursive
```

To build the examples run
```
cargo build --all --examples
```

# run the examples
Open three terminals
- start RouDi `target/iceoryx-install/bin/RouDi`
- start the publisher `target/debug/examples/publisher_simple`
- start a subscriber `target/debug/examples/subscriber_simple`
