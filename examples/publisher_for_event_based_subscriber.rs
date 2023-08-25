// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: © Contributors to the iceoryx-rs project
// SPDX-FileContributor: Mathias Kraus

use iceoryx_rs::PublisherBuilder;
use iceoryx_rs::Runtime;

use std::error::Error;
use std::thread;
use std::time::Duration;

fn main() -> Result<(), Box<dyn Error>> {
    Runtime::init("fibonacci");

    let publisher = PublisherBuilder::<u64>::new("math", "sequence", "fibonacci").create()?;

    let mut fib_current = 0u64;
    let mut fib_next = 1u64;
    loop {
        let mut sample = publisher.loan()?;
        *sample = fib_current;
        publisher.publish(sample);

        println!("Sending: {}", fib_current);
        let fib_next_new = fib_current + fib_next;
        fib_current = fib_next;
        fib_next = fib_next_new;

        thread::sleep(Duration::from_millis(1000));
    }
}
