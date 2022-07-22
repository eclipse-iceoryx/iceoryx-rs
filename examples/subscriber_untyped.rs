// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: © Contributors to the iceoryx-rs project
// SPDX-FileContributor: Mathias Kraus

mod topic;
use topic::Counter;

use iceoryx_rs::Runtime;
use iceoryx_rs::SubscriberBuilder;

use bytes::Buf;

use std::error::Error;
use std::thread;
use std::time::Duration;

fn main() -> Result<(), Box<dyn Error>> {
    Runtime::init("subscriber_untyped");

    let (subscriber, sample_receive_token) =
        SubscriberBuilder::<[u8]>::new("Radar", "FrontLeft", "Counter")
            .queue_capacity(5)
            .create()?;

    let sample_receiver = subscriber.get_sample_receiver(sample_receive_token);

    let mut counter = 0;
    loop {
        if sample_receiver.has_data() {
            while let Some(sample) = sample_receiver.take() {
                counter = match counter % 2 {
                    // as buffer
                    0 => sample.as_ref().get_u32_le(),
                    // transmute to concrete type
                    1 => unsafe { sample.try_as::<Counter>().expect("Valid data").counter },
                    _ => unreachable!(),
                };
                println!("Receiving: {}", sample.as_ref().get_u32_le());
            }
        } else {
            thread::sleep(Duration::from_millis(100));
        }
    }
}
