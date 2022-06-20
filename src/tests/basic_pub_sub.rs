// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: © Contributors to the iceoryx-rs project
// SPDX-FileContributor: Mathias Kraus

use crate::pb::{self, POD};
use crate::sb;
use crate::testing::RouDiEnvironment;
use crate::Runtime;

use anyhow::{anyhow, Result};

#[repr(C)]
struct CounterTopic {
    counter: u32,
}

unsafe impl POD for CounterTopic {}

#[test]
fn basic_pub_sub() -> Result<()> {
    let _roudi = RouDiEnvironment::new();

    Runtime::init("basic_pub_sub");

    let topic = sb::TopicBuilder::<CounterTopic>::new("Test", "BasicPubSub", "Counter")
        .queue_capacity(5)
        .build();

    let (subscriber, sample_receive_token) = topic.subscribe();

    let topic = pb::TopicBuilder::<CounterTopic>::new("Test", "BasicPubSub", "Counter").build()?;

    let publisher = topic.offer();
    let mut sample = publisher.allocate_sample()?;
    const SEND_COUNTER: u32 = 42;
    sample.counter = SEND_COUNTER;
    publisher.publish(sample);

    let sample_receiver = subscriber.get_sample_receiver(sample_receive_token);

    assert!(sample_receiver.has_samples());

    match sample_receiver.get_sample() {
        Some(sample) => assert_eq!(sample.counter, SEND_COUNTER),
        _ => return Err(anyhow!("Could not read sample")),
    }

    Ok(())
}
