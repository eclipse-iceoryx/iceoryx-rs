// Copyright 2020 Mathias Kraus - All rights reserved
//
// Licensed under the Apache License, Version 2.0 <LICENSE or
// http://www.apache.org/licenses/LICENSE-2.0>. This file may not be
// copied, modified, or distributed except according to those terms.

use iceoryx_rs::pb::{Topic, POD};
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
    Runtime::get_instance("/publisher_simple");

    let topic = Topic::<CounterTopic>::new("Radar", "FrontLeft", "Counter");

    let publisher = topic.offer();
    // wait until RouDi runs the discovery loop
    thread::sleep(Duration::from_millis(100));

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
