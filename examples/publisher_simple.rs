// Copyright 2020 Mathias Kraus. All rights reserved.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use iceoryx_rs::{Publisher, Runtime, POD};

use std::error::Error;
use std::thread;
use std::time::Duration;

#[repr(C)]
struct CounterTopic {
    counter: u32,
}

unsafe impl POD for CounterTopic {}

fn main() -> Result<(), Box<dyn Error>> {
    Runtime::get_intance("/publisher_simple");

    let publisher = Publisher::<CounterTopic>::new("Radar", "FrontLeft", "Counter");

    let service = publisher.offer();
    // wait until RouDi runs the discovery loop
    thread::sleep(Duration::from_millis(100));

    let mut counter = 0u32;
    loop {
        let mut sample = service.allocate_sample()?;
        sample.counter = counter;
        service.publish(sample);

        println!("Sending: {}", counter);
        counter += 1;

        thread::sleep(Duration::from_millis(1000));
    }
}
