// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: © Contributors to the iceoryx-rs project
// SPDX-FileContributor: Mathias Kraus

use iceoryx_rs::sb::{SampleReceiverWaitState, SubscriberBuilder};
use iceoryx_rs::Runtime;

use std::error::Error;
use std::thread;
use std::time::Duration;

#[repr(C)]
struct Counter {
    counter: u32,
}

fn main() -> Result<(), Box<dyn Error>> {
    Runtime::init("subscriber_multithreaded");

    let (subscriber, sample_receive_token) =
        SubscriberBuilder::<Counter>::new("Radar", "FrontLeft", "Counter")
            .queue_capacity(5)
            .create_mt()?;

    let sample_receiver = subscriber.get_sample_receiver(sample_receive_token);

    let th = thread::spawn(move || {
        loop {
            match sample_receiver.wait_for_samples(Duration::from_secs(2)) {
                SampleReceiverWaitState::SamplesAvailable => {
                    while let Some(sample) = sample_receiver.get_sample() {
                        println!("Receiving: {}", sample.counter);
                    }
                }
                SampleReceiverWaitState::Timeout => {
                    println!("Timeout while waiting for samples!");
                    break;
                }
                SampleReceiverWaitState::Stopped => break,
            }
        }

        sample_receiver
    });

    let sample_receiver = th.join().map_err(|_| "could not join threads")?;
    subscriber.unsubscribe(sample_receiver);

    Ok(())
}
