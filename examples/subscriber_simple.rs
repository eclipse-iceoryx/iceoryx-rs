// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: © Contributors to the iceoryx-rs project
// SPDX-FileContributor: Mathias Kraus

mod topic;
use topic::Counter;

use iceoryx_rs::Runtime;
use iceoryx_rs::SubscriberBuilder;

use std::error::Error;
use std::thread;
use std::time::Duration;

fn main() -> Result<(), Box<dyn Error>> {
    Runtime::init("subscriber_simple");

    let (subscriber, sample_receive_token) =
        SubscriberBuilder::<Counter>::new("Radar", "FrontLeft", "Counter")
            .queue_capacity(5)
            .create()?;

    let sample_receiver = subscriber.get_sample_receiver(sample_receive_token);

    loop {
        if sample_receiver.has_data() {
            while let Some(sample) = sample_receiver.take() {
                println!("Receiving: {}", sample.counter);
            }
        } else {
            thread::sleep(Duration::from_millis(100));
        }
    }
}
