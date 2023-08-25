// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: © Contributors to the iceoryx-rs project
// SPDX-FileContributor: Mathias Kraus

mod topic;
use topic::Counter;

use iceoryx_rs::Runtime;
use iceoryx_rs::SubscriberBuilder;
use iceoryx_rs::SampleReceiver;
use iceoryx_rs::reactor::{self, Control, Condition, Dispatcher, Foo};

use std::error::Error;
use std::thread;
use std::time::Duration;

fn main() -> Result<(), Box<dyn Error>> {
    Runtime::init("subscriber_simple");

    let (counter_subscriber, counter_sample_receive_token) =
        SubscriberBuilder::<Counter>::new("Radar", "FrontLeft", "Counter")
            .queue_capacity(5)
            .create()?;

    let counter_sample_receiver = counter_subscriber.get_sample_receiver(counter_sample_receive_token);

    let r = reactor::Reactor::new();
    let (mut control, mut dispatcher) = r.split();
    let mut token = control.attach(counter_sample_receiver, Box::new(|s| { Condition::State(s.has_data()) }), Box::new(|s|
        {
            while let Some(sample) = s.take() {
                println!("Receiving counter: {}", sample.counter);
            }
        })
    );

    dispatcher.add_handler(token.handler.take().unwrap());

    let (fibonacci_subscriber, fibonacci_sample_receive_token) = SubscriberBuilder::<u64>::new("math", "sequence", "fibonacci").queue_capacity(5).create()?;
    let fibonacci_sample_receiver = fibonacci_subscriber.get_sample_receiver(fibonacci_sample_receive_token);
    let mut token = control.attach(fibonacci_sample_receiver, Box::new(|s| { Condition::State(s.has_data()) }), Box::new(|s|
        {
            while let Some(sample) = s.take() {
                println!("Receiving fibonacci: {}", *sample);
            }
        })
    );

    dispatcher.add_handler(token.handler.take().unwrap());

    while let Some(notification_index) = dispatcher.next_with_timeout(Duration::from_secs(2)) {
        println!("notification index: {}", notification_index);
    }

//     loop {
//         if sample_receiver.has_data() {
//             while let Some(sample) = sample_receiver.take() {
//                 println!("Receiving: {}", sample.counter);
//             }
//         } else {
//             thread::sleep(Duration::from_millis(100));
//         }
//     }

    Ok(())
}
