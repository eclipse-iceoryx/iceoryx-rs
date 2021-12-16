// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: © Contributors to the iceoryx-rs project
// SPDX-FileContributor: Mathias Kraus

use iceoryx_rs::pb::{TopicBuilder, POD};
use iceoryx_rs::Runtime;

use std::error::Error;
use std::thread;
use std::time::Duration;

#[repr(C)]
struct CounterTopic {
    counter: u32,
}

unsafe impl POD for CounterTopic {}

fn main() -> Result<(), Box<dyn Error>> {
    Runtime::init("publisher_simple");

    let topic = TopicBuilder::<CounterTopic>::new("Radar", "FrontLeft", "Counter").build()?;

    let publisher = topic.offer();

    // wait until RouDi runs the discovery loop
    while !publisher.is_offered() {
        thread::sleep(Duration::from_millis(10));
    }

    let mut counter = 0u32;
    loop {
        let mut sample = publisher.allocate_sample()?;
        sample.counter = counter;
        publisher.publish(sample);

        println!("Sending: {}", counter);
        counter += 1;

        thread::sleep(Duration::from_millis(1000));
    }
}
