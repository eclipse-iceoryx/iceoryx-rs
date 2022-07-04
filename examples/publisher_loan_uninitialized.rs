// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: © Contributors to the iceoryx-rs project
// SPDX-FileContributor: Mathias Kraus

mod topic;
use topic::Counter;

use iceoryx_rs::PublisherBuilder;
use iceoryx_rs::Runtime;

use std::error::Error;
use std::thread;
use std::time::Duration;

fn main() -> Result<(), Box<dyn Error>> {
    Runtime::init("publisher_simple");

    let publisher = PublisherBuilder::<Counter>::new("Radar", "FrontLeft", "Counter").create()?;

    let mut counter = 0u32;
    loop {
        let mut sample = publisher.loan_uninitialized()?;
        let sample = unsafe {
            (*sample.as_mut_ptr()).counter = counter;
            sample.assume_init()
        };
        publisher.publish(sample);

        println!("Sending: {}", counter);
        counter += 1;

        thread::sleep(Duration::from_millis(1000));
    }
}
