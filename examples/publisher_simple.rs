// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: © Contributors to the iceoryx-rs project
// SPDX-FileContributor: Mathias Kraus

use iceoryx_rs::marker::ShmSend;
use iceoryx_rs::PublisherBuilder;
use iceoryx_rs::Runtime;

use std::error::Error;
use std::thread;
use std::time::Duration;

#[repr(C)]
#[derive(Default)]
struct Counter {
    counter: u32,
}

unsafe impl ShmSend for Counter {}

fn main() -> Result<(), Box<dyn Error>> {
    Runtime::init("publisher_simple");

    let publisher = PublisherBuilder::<Counter>::new("Radar", "FrontLeft", "Counter").create()?;

    let mut counter = 0u32;
    loop {
        let mut sample = publisher.loan()?;
        sample.counter = counter;
        publisher.publish(sample);

        println!("Sending: {}", counter);
        counter += 1;

        thread::sleep(Duration::from_millis(1000));
    }
}
