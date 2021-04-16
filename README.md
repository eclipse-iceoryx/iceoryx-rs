# iceoryx-rs

<p align="center">
<img src="https://user-images.githubusercontent.com/8661268/114321508-64a6b000-9b1b-11eb-95ef-b84c91387cff.png" width="50%">
</p>

Experimental rust wrapper for the [iceoryx](https://github.com/eclipse-iceoryx/iceoryx) IPC middleware.

# clone and build

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
- start RouDi `target/iceoryx-install/bin/iox-roudi`
- start the publisher `target/debug/examples/publisher_simple`
- start a subscriber `target/debug/examples/subscriber_simple`
