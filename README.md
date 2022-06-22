# iceoryx-rs

[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![Build & Test](https://github.com/eclipse-iceoryx/iceoryx-rs/workflows/Build%20&%20Test/badge.svg?branch=master)](https://github.com/eclipse-iceoryx/iceoryx-rs/actions)
[![Codecov](https://codecov.io/gh/eclipse-iceoryx/iceoryx-rs/branch/master/graph/badge.svg?branch=master)](https://codecov.io/gh/eclipse-iceoryx/iceoryx-rs?branch=master)

<p align="center">
<img src="https://user-images.githubusercontent.com/8661268/114321508-64a6b000-9b1b-11eb-95ef-b84c91387cff.png" width="50%">
</p>

Experimental rust wrapper for the [iceoryx](https://github.com/eclipse-iceoryx/iceoryx) IPC middleware.

# clone and build

Clone the repository with
```
git clone https://github.com/elBoberido/iceoryx-rs.git
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
