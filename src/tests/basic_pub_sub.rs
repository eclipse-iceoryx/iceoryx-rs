// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: © Contributors to the iceoryx-rs project
// SPDX-FileContributor: Mathias Kraus

use crate::marker::ShmSend;
use crate::testing::RouDiEnvironment;
use crate::PublisherBuilder;
use crate::Runtime;
use crate::SubscriberBuilder;

use anyhow::{anyhow, Result};
use bytes::{Buf, BufMut};

use std::thread;

#[repr(C)]
#[derive(Default)]
struct Counter {
    counter: u32,
}

unsafe impl ShmSend for Counter {}

#[test]
fn single_threaded_subscriber() -> Result<()> {
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

    publisher.stop_offer();
    subscriber.unsubscribe(sample_receiver);

    Ok(())
}

#[test]
fn multi_threaded_subscriber() -> Result<()> {
    let _roudi = RouDiEnvironment::new();

    Runtime::init("basic_pub_sub");

    let (subscriber, sample_receive_token) =
        SubscriberBuilder::<Counter>::new("Test", "BasicPubSub", "Counter")
            .queue_capacity(5)
            .create_mt()?;

    let publisher = PublisherBuilder::<Counter>::new("Test", "BasicPubSub", "Counter").create()?;

    let mut sample = publisher.loan()?;

    const SEND_COUNTER: u32 = 13;
    sample.counter = SEND_COUNTER;
    publisher.publish(sample);

    let sample_receiver = subscriber.get_sample_receiver(sample_receive_token);

    let th = thread::spawn(move || {
        assert!(sample_receiver.has_data());

        match sample_receiver.take() {
            Some(sample) => assert_eq!(sample.counter, SEND_COUNTER),
            _ => assert!(false, "no sample received"),
        }

        sample_receiver
    });

    let sample_receiver = th.join().map_err(|_| anyhow!("could not join threads"))?;

    publisher.stop_offer();
    subscriber.unsubscribe(sample_receiver);

    Ok(())
}

#[test]
fn loan_sample_but_not_publish() -> Result<()> {
    let _roudi = RouDiEnvironment::new();

    Runtime::init("basic_pub_sub");

    let (subscriber, sample_receive_token) =
        SubscriberBuilder::<Counter>::new("Test", "BasicPubSub", "Counter")
            .queue_capacity(5)
            .create()?;

    let publisher = PublisherBuilder::<Counter>::new("Test", "BasicPubSub", "Counter").create()?;

    {
        let _sample = publisher.loan()?;
    }

    let sample_receiver = subscriber.get_sample_receiver(sample_receive_token);

    assert!(!sample_receiver.has_data());

    publisher.stop_offer();
    subscriber.unsubscribe(sample_receiver);

    Ok(())
}

#[test]
fn loan_uninit_sample_and_publish() -> Result<()> {
    let _roudi = RouDiEnvironment::new();

    Runtime::init("basic_pub_sub");

    let (subscriber, sample_receive_token) =
        SubscriberBuilder::<Counter>::new("Test", "BasicPubSub", "Counter")
            .queue_capacity(5)
            .create()?;

    let publisher = PublisherBuilder::<Counter>::new("Test", "BasicPubSub", "Counter").create()?;

    const SEND_COUNTER: u32 = 73;
    let mut sample = publisher.loan_uninit()?;
    let sample = unsafe {
        (*sample.as_mut_ptr()).counter = SEND_COUNTER;
        sample.assume_init()
    };
    publisher.publish(sample);

    let sample_receiver = subscriber.get_sample_receiver(sample_receive_token);

    assert!(sample_receiver.has_data());

    match sample_receiver.take() {
        Some(sample) => assert_eq!(sample.counter, SEND_COUNTER),
        _ => return Err(anyhow!("Could not read sample")),
    }

    publisher.stop_offer();
    subscriber.unsubscribe(sample_receiver);

    Ok(())
}

#[test]
fn loan_slice_and_publish() -> Result<()> {
    let _roudi = RouDiEnvironment::new();

    Runtime::init("basic_pub_sub");

    let (subscriber, sample_receive_token) =
        SubscriberBuilder::<[u8]>::new("Test", "BasicPubSub", "Counter")
            .queue_capacity(5)
            .create()?;

    let publisher = PublisherBuilder::<[u8]>::new("Test", "BasicPubSub", "Counter").create()?;

    const SEND_COUNTER: u32 = 37;
    let mut sample = publisher
        .loan_slice_with_alignment(std::mem::size_of::<u32>(), std::mem::align_of::<u32>())?;
    sample.as_mut().put_u32_le(SEND_COUNTER);
    publisher.publish(sample);

    let sample_receiver = subscriber.get_sample_receiver(sample_receive_token);

    assert!(sample_receiver.has_data());

    match sample_receiver.take() {
        Some(sample) => assert_eq!(sample.as_ref().get_u32_le(), SEND_COUNTER),
        _ => return Err(anyhow!("Could not read sample")),
    }

    publisher.stop_offer();
    subscriber.unsubscribe(sample_receiver);

    Ok(())
}

#[test]
fn loan_uninit_slice_and_publish() -> Result<()> {
    let _roudi = RouDiEnvironment::new();

    Runtime::init("basic_pub_sub");

    let (subscriber, sample_receive_token) =
        SubscriberBuilder::<[Counter]>::new("Test", "BasicPubSub", "Counter")
            .queue_capacity(5)
            .create()?;

    let publisher = PublisherBuilder::<[u8]>::new("Test", "BasicPubSub", "Counter").create()?;

    const SEND_COUNTER: u32 = 3773;
    let mut sample = publisher.loan_uninit_slice_with_alignment(
        std::mem::size_of::<u32>(),
        std::mem::align_of::<u32>(),
    )?;
    let sample = unsafe {
        sample.slice_assume_init_mut().put_u32_le(SEND_COUNTER);
        sample.assume_init()
    };
    publisher.publish(sample);

    let sample_receiver = subscriber.get_sample_receiver(sample_receive_token);

    assert!(sample_receiver.has_data());

    match sample_receiver.take() {
        Some(sample) => {
            assert_eq!(sample.len(), 1);
            assert_eq!(sample[0].counter, SEND_COUNTER);
        }
        _ => return Err(anyhow!("Could not read sample")),
    }

    publisher.stop_offer();
    subscriber.unsubscribe(sample_receiver);

    Ok(())
}
