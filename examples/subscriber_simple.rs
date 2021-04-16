// SPDX-License-Identifier: Apache-2.0

use iceoryx_rs::sb::{SubscribeState, TopicBuilder};
use iceoryx_rs::Runtime;

use std::thread;
use std::time::Duration;

#[repr(C)]
struct CounterTopic {
    counter: u32,
}

fn main() {
    Runtime::init("subscriber_simple");

    let topic = TopicBuilder::<CounterTopic>::new("Radar", "FrontLeft", "Counter")
        .queue_capacity(5)
        .build();

    let (subscriber, sample_receive_token) = topic.subscribe();

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

    loop {
        if sample_receiver.has_samples() {
            while let Some(sample) = sample_receiver.get_sample() {
                println!("Receiving: {}", sample.counter);
            }
        } else {
            thread::sleep(Duration::from_millis(100));
        }
    }
}
