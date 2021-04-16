// SPDX-License-Identifier: Apache-2.0

use iceoryx_rs::sb::{SampleReceiverWaitState, SubscribeState, TopicBuilder};
use iceoryx_rs::Runtime;

use std::error::Error;
use std::thread;
use std::time::Duration;

#[repr(C)]
struct CounterTopic {
    counter: u32,
}

fn main() -> Result<(), Box<dyn Error>> {
    Runtime::init("subscriber_multithreaded");

    let topic = TopicBuilder::<CounterTopic>::new("Radar", "FrontLeft", "Counter")
        .queue_capacity(5)
        .build();

    let (subscriber, sample_receive_token) = topic.subscribe_mt();

    let mut has_printed_waiting_for_subscription = false;
    while subscriber.subscription_state() != SubscribeState::Subscribed {
        if !has_printed_waiting_for_subscription {
            println!("waiting for subscription ...");
            has_printed_waiting_for_subscription = true;
        }
        thread::sleep(Duration::from_millis(10));
    }

    if has_printed_waiting_for_subscription {
        println!("  -> subscribed");
    }

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
