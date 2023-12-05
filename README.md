# iceoryx-rs

[![Crates.io](https://img.shields.io/crates/v/iceoryx-rs.svg)](https://crates.io/crates/iceoryx-rs)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![Build & Test](https://img.shields.io/github/actions/workflow/status/eclipse-iceoryx/iceoryx-rs/rust.yml?event=push&label=Build%20%26%20Test)](https://github.com/eclipse-iceoryx/iceoryx-rs/actions)
[![Codecov](https://codecov.io/gh/eclipse-iceoryx/iceoryx-rs/branch/master/graph/badge.svg?branch=master)](https://codecov.io/gh/eclipse-iceoryx/iceoryx-rs?branch=master)

<p align="center">
<img src="https://user-images.githubusercontent.com/8661268/114321508-64a6b000-9b1b-11eb-95ef-b84c91387cff.png" width="50%">
</p>

Safe Rust bindings for [Eclipse iceoryx](https://github.com/eclipse-iceoryx/iceoryx).

1. [About](#about)
2. [Examples](#examples)
    - [How to start RouDi](#how-to-start-roudi)
    - [Run the simple publisher and subscriber example](#run-the-simple-publisher-and-subscriber-example)
3. [How to write a simple application](#how-to-write-a-simple-application)
4. [Cross-compiling](#cross-compiling)
5. [Limitations](#limitations)

## About

Eclipse iceoryx is a true zero-copy, inter-process communication framework with the goal to boost
autonomous driving with their demand on high data throughput and low latency. With its bandwidth and speed,
iceoryx also fits well into other domains where low latency and transmitting large data structures
is a concern. If you would like to know more about Eclipse iceoryx you can take a look at the
`Getting started` section on [iceoryx.io](https://iceoryx.io) or the
[README.md](https://github.com/eclipse-iceoryx/iceoryx/blob/master/README.md) of the main project.

The Rust bindings are a work in progress and currently support only the pub-sub messaging pattern.
Upcoming releases will close the gap and the goal is to have the Rust bindings as a first class citizen
in the iceoryx ecosystem.

This project started with the goal to create an awesome looking introspection TUI in Rust and led to
[iceray](https://crates.io/crates/iceray). Check it out.

## Examples

Before you can run the example you have to build them first with
```console
cargo build --all --examples
```

In order to run an iceoryx application, the `RouDi` daemon needs to run.

### How to start RouDi

`RouDi` is the central daemon which takes care of resource management and connects the services
when they register. After the registration phase it is not involved in the communication anymore.

You can find more information about `RouDi` [here](https://iceoryx.io/v2.0.2/getting-started/overview/#roudi)
and if you are more of a visual person, just scroll up a little bit on that page to view an amazing animation.

In case you have iceoryx installed on your system, you can use the `iox-roudi` binary from that installation.
If that's not the case, you can run `RouDi` with the following command from the root of your crate.

```console
find target -type f -wholename "*/iceoryx-install/bin/iox-roudi" -exec {} \;
```

### Run the simple publisher and subscriber example

The `publisher_simple` and `subscriber_simple` examples are demonstrating a typical inter-process communication
use case.

A good introductory example to demonstrate the inter-process communication are `publisher_simple`
and `subscriber_simple`.

You can run the publisher and subscriber by executing the following commands in separate terminals

```console
cargo run --example publisher_simple
cargo run --example subscriber_simple
```

You should see the messages sent by the publisher like

```console
Sending: 0
Sending: 1
Sending: 2
Sending: 3
Sending: 4
```

and how they are received by the subscriber

```console
Receiving: 0
Receiving: 1
Receiving: 2
Receiving: 3
Receiving: 4
```

You might also witness this output

```console
[Warning]: IPC channel still there, doing an unlink of [app_name]
```

Don't worry about this. This is due to aborting a previous run with `Ctrl+C`.
To prevent this output one has to register a signal handler and gracefully shut down
the application. The `RouDi` daemon will automatically clean up the shared resources
of an abnormally terminated application. An application cleans its own leftover on restart
up as well, hence the output.

If `RouDi` is not running you get this output.

```console
[Warning]: RouDi not found - waiting ...
```

After a waiting period, the application will shut down.

## How to write a simple application

This is a brief API guide how to write a simple application.

We start with `cargo new`.


```console
cargo new --bin hypnotoad
```

In the `Cargo.toml` manifest file we create two binaries and add the `iceoryx-rs` dependency.

```toml
[[bin]]
name = "publisher"
path = "src/publisher.rs"

[[bin]]
name = "subscriber"
path = "src/subscriber.rs"

[dependencies]
iceoryx-rs = "0.1"
```

Now lets define the data we want to transmit and store them in `src/topic.rs`.

```rust
use iceoryx_rs::marker::ShmSend;

#[repr(C)]
#[derive(Default)]
pub struct Counter {
    pub counter: u32,
}

unsafe impl ShmSend for Counter {}
```

The `ShmSend` marker trait is used for types that can be transferred via shared memory and is similar
to the `Send` marker trait which is used for types that can be transferred across thread boundaries.

The types which implement `ShmSend` must satisfy the following constraints:
 - no heap is used
 - the data structure is entirely contained in the shared memory - no pointers
   to process local memory, no references to process local constructs, no dynamic allocators
 - the data structure has to be relocatable and therefore must not internally
   use pointers/references
 - the type must not implement `Drop`; `drop` will not be called when the memory is released since the
   memory might be located in a shm segment without write access to the subscriber
In general, types that could implement the Copy trait fulfill these requirements.

The data type has also the `#[repr(C)]` attribute to be able to communicate with C and C++ applications
and implements the `Default` trait. If the `Default` trait is not implemented, an `unsafe` API must
be used to loan samples.

Next is the `src/publisher.rs` file.

```rust
use iceoryx_rs::PublisherBuilder;
use iceoryx_rs::Runtime;

use std::error::Error;
use std::thread;
use std::time::Duration;

mod topic;
use topic::Counter;

fn main() -> Result<(), Box<dyn Error>> {
    Runtime::init("publisher");

    let publisher = PublisherBuilder::<Counter>::new("all", "glory", "hypnotoad").create()?;

    let mut counter = 0u32;
    loop {
        let mut sample = publisher.loan()?;
        sample.counter = counter;
        publisher.publish(sample);

        println!("Send praise hypnotoad: {}", counter);
        counter += 1;

        thread::sleep(Duration::from_millis(1000));
    }
}
```

The first thing to do is the initialization of the iceoryx `Runtime`. This does the registration at the
central `RouDi` daemon and takes the application name as parameter.

Then the `Publisher` is created with the `PublisherBuilder` by specifying a service, event and instance ID.
These can be arbitrary strings and are used to match publisher and subscriber.

The publisher code is completed with a loop where once a second a sample is loaned and published.

Finally we create the `src/subsriber.rs` file.

```rust
use iceoryx_rs::Runtime;
use iceoryx_rs::SubscriberBuilder;

use std::error::Error;
use std::thread;
use std::time::Duration;

mod topic;
use topic::Counter;

fn main() -> Result<(), Box<dyn Error>> {
    Runtime::init("subscriber");

    let (subscriber, sample_receive_token) =
        SubscriberBuilder::<Counter>::new("all", "glory", "hypnotoad")
            .queue_capacity(5)
            .create()?;

    let sample_receiver = subscriber.get_sample_receiver(sample_receive_token);

    loop {
        if let Some(sample) = sample_receiver.take() {
            println!("Receiving praise hypnotoad: {}", sample.counter);
        } else {
            thread::sleep(Duration::from_millis(100));
        }
    }
}
```

Similar to the publisher application, the first thing to do is initializing the `Runtime`.

The `SubscriberBuilder` is used to create a `Subscriber` and a `SampleReceiveToken` by specifying the
same three service, event and instance ID strings as with the publisher.

Before entering the loop, a `SampleReceiver` is obtained. In the loop we `take` samples until the
receiver queue is empty and print the data we received. In case there is no data, the thread is
suspended for one second and we try to take new samples.

We are done. Lets run our code.

1. Start `RouDi` with `find target -type f -wholename "*/iceoryx-install/bin/iox-roudi" -exec {} \;`
2. Start the `publisher` with `cargo run publisher`
2. Start the `subscriber` with `cargo run subscriber`

Please have a look at the [examples](https://github.com/eclipse-iceoryx/iceoryx-rs/tree/master/examples)
in the repository. It contains additional examples to show how uninitialized samples can be loaned and
how the `wait_for_samples` method of the `SampleReceiver` can be used to get notified on new samples.

## Cross-Compiling

`iceoryx-sys` uses `cmake` to build `iceoryx`, which has a dependency to `libacl` on Linux. As a result,
to link to a cross-compiled version of the native `iceoryx`, it's necessary to set several environment variables
to link against the prebuilt libacl.

To tell the linker lookup libraries from your sysroot, please set the following environment variables, and invoke the build script:

```bash
# let's take the target aarch64-unknown-linux-gnu for instance
SYSROOT=/path/to/your/cross/compile/sysroot
export LDFLAGS="--sysroot $SYSROOT"
export CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_RUSTFLAGS="-C link-arg=--sysroot=$SYSROOT"

cargo build --target aarch64-unknown-linux-gnu --all-targets
```

## Limitations

Currently, only a subset of Eclipse iceoryx v2.0 is supported and some features are missing.

Supported:
- pub-sub messaging pattern
- accessing introspection topics like memory usage and available publisher and subscriber
    - have a look at [iceray](https://crates.io/crates/iceray)

Missing:
- user defined header for pub-sub data
- request-response messaging pattern
- `Listener` and `WaitSet`
- lookup of available services aka `ServiceDiscovery`
