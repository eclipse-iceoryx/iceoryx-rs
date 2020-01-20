// Copyright 2020 Mathias Kraus - All rights reserved
//
// Licensed under the Apache License, Version 2.0 <LICENSE or
// http://www.apache.org/licenses/LICENSE-2.0>. This file may not be
// copied, modified, or distributed except according to those terms.

use iceoryx_rs::{Runtime, Subscriber, SubscriptionState};

use std::thread;
use std::time::Duration;

#[repr(C)]
struct CounterTopic {
    counter: u32,
}

fn main() {
    Runtime::get_intance("/subscriber_simple");

    let subscriber = Subscriber::<CounterTopic>::new("Radar", "FrontLeft", "Counter");

    const CACHE_SIZE: u32 = 5;
    let (recipient, sample_receive_token) = subscriber.subscribe(CACHE_SIZE);

    let mut has_printed_waiting_for_subscription = false;
    while recipient.subscription_state() != SubscriptionState::Subscribed {
        if !has_printed_waiting_for_subscription {
            println!("waiting for subscription ...");
            has_printed_waiting_for_subscription = true;
        }
        thread::sleep(Duration::from_millis(10));
    }

    if has_printed_waiting_for_subscription {
        println!("  -> subscribed");
    }

    let sample_receiver = recipient.get_sample_receiver(sample_receive_token);

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
