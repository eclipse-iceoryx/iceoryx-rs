# iceoryx-rs

[![Crates.io](https://img.shields.io/crates/v/iceoryx-rs.svg)](https://crates.io/crates/iceoryx-rs)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![Build & Test](https://img.shields.io/github/workflow/status/eclipse-iceoryx/iceoryx-rs/Build%20&%20Test/master?label=Build%20%26%20Test)](https://github.com/eclipse-iceoryx/iceoryx-rs/actions)
[![Codecov](https://codecov.io/gh/eclipse-iceoryx/iceoryx-rs/branch/master/graph/badge.svg?branch=master)](https://codecov.io/gh/eclipse-iceoryx/iceoryx-rs?branch=master)

<p align="center">
<img src="https://user-images.githubusercontent.com/8661268/114321508-64a6b000-9b1b-11eb-95ef-b84c91387cff.png" width="50%">
</p>

Safe Rust bindings to [Eclipse iceoryx](https://github.com/eclipse-iceoryx/iceoryx).

1. [About](#about)
2. [Examples](#examples)
    - [How to start RouDi](#how-to-start-roudi)
    - [Run the simple publisher and subscriber example](#run-the-simple-publisher-and-subscriber-example)
3. [Limitations](#limitations)

## About

Eclipse iceoryx is a true zero-copy, inter-process communication framework with the goal to boost
autonomous driving with their demand on high data throughput and low latency. With this properties,
iceoryx also fits well into other domains where low latency and transmitting large data structures
is a concern. For a closer look about Eclipse iceoryx actually is, pleaes have a look at
`Getting started` section on [iceoryx.io](https://iceoryx.io) or the
[README.md](https://github.com/eclipse-iceoryx/iceoryx/blob/master/README.md) of the main project.

The Rust bindings are a work in progress and and currently support only the pub-sub messaging pattern.
Upcoming release will close the gap and the goal is to have the Rust bindings as first class citizen
in the iceoryx ecosystem.

This project started with the goal to create an nice looking introspection TUI in Rust and led to
[iceray](https://crates.io/crates/iceray). Check it out.

## Examples

Before you can run the example you have to build them first with
```
cargo build --all --examples
```

In order to run an iceoryx application, the `RouDi` deamon needs to run.

### How to start RouDi

`RouDi` is the cetral deamon which takes care of resource management and connects the services
when they register. After the registration phase it is not involved in the communication anymore.

You can find more information [here](https://iceoryx.io/v2.0.2/getting-started/overview/#roudi)
and if you are more of a visual person, just scroll up a little bit on that page to view a nice animation.

In case you have iceoryx installed on your system, you can use the `iox-roudi` binary from that installation.
If that's not the case, you can run `RouDi` with the following command from the root of your crate.

```
find target -type f -wholename "*/iceoryx-install/bin/iox-roudi" -exec {} \;
```

### Run the simple publisher and subscriber example

The most simple examples are the `publisher_simple` and `subscriber_simple`.

You can run them by executing the following commands in separate terminals

```
cargo run --example publisher_simple
cargo run --example subscriber_simple
```

You should see something like this for the publisher
```
Sending: 0
Sending: 1
Sending: 2
Sending: 3
Sending: 4
```

and this for the subscriber

```
Receiving: 0
Receiving: 1
Receiving: 2
Receiving: 3
Receiving: 4
```

You might also see this output

```
[Warning]: IPC channel still there, doing an unlink of [app_name]
```

Don't worry about this. This is due to aborting a previous run with `Ctrl+C`.
To prevent this output one has to register a signal handler and gracefully shut down
the application. The `RouDi` daemon will automatically clean up the share resources
of an abnormally terminated application and on restart the app will do the same for
its own leftover.

If `RouDi` is not running you get this output.

```
[Warning]: RouDi not found - waiting ...
```

After a waiting period, the application will shut down.

## Limitations

Currently only a subset of Eclipse iceoryx v2.0 is supported and some features are missing.

Supported:
- pub-sub messaging pattern
- accessing introspection topics like memory usage and available publisher and subscriber
    - have a look at [iceray](https://crates.io/crates/iceray)

Missing:
- user defined header for pub-sub data
- request-response messaging pattern
- `Listener` and `WaitSet`
- lookup of available services aka `ServiceDiscovery`
