// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: © Contributors to the iceoryx-rs project
// SPDX-FileContributor: Mathias Kraus

use crate::marker::ShmSend;
use crate::testing::RouDiEnvironment;
use crate::PublisherBuilder;
use crate::Runtime;
use crate::SubscriberBuilder;

use anyhow::{anyhow, Result};

#[repr(C)]
#[derive(Default)]
struct Counter {
    counter: u32,
}

unsafe impl ShmSend for Counter {}

#[test]
fn basic_pub_sub() -> Result<()> {
    let _roudi = RouDiEnvironment::new();

    Runtime::init("basic_pub_sub");

    let (subscriber, sample_receive_token) =
        SubscriberBuilder::<Counter>::new("Test", "BasicPubSub", "Counter")
            .queue_capacity(5)
            .create()?;

    let publisher = PublisherBuilder::<Counter>::new("Test", "BasicPubSub", "Counter").create()?;

    let mut sample = publisher.loan()?;

    const SEND_COUNTER: u32 = 42;
    sample.counter = SEND_COUNTER;
    publisher.publish(sample);

    let sample_receiver = subscriber.get_sample_receiver(sample_receive_token);

    assert!(sample_receiver.has_data());

    match sample_receiver.take() {
        Some(sample) => assert_eq!(sample.counter, SEND_COUNTER),
        _ => return Err(anyhow!("Could not read sample")),
    }

    Ok(())
}
