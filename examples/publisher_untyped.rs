// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: © Contributors to the iceoryx-rs project
// SPDX-FileContributor: Mathias Kraus

use iceoryx_rs::PublisherBuilder;
use iceoryx_rs::Runtime;

use bytes::BufMut;

use std::error::Error;
use std::thread;
use std::time::Duration;

fn main() -> Result<(), Box<dyn Error>> {
    Runtime::init("publisher_untyped");

    let publisher = PublisherBuilder::<[u8]>::new("Radar", "FrontLeft", "Counter").create()?;

    let mut counter = 0u32;
    loop {
        let sample = if counter % 2 == 1 {
            // with initialized slice
            let mut sample = publisher.loan_slice_with_alignment(
                std::mem::size_of::<u32>(),
                std::mem::align_of::<u32>(),
            )?;
            sample.as_mut().put_u32_le(counter);
            sample
        } else {
            // with uninitialized slice
            let mut sample = publisher.loan_uninit_slice_with_alignment(
                std::mem::size_of::<u32>(),
                std::mem::align_of::<u32>(),
            )?;
            unsafe {
                sample.slice_assume_init_mut().put_u32_le(counter);
                sample.assume_init()
            }
        };
        publisher.publish(sample);

        println!("Sending: {}", counter);
        counter += 1;

        thread::sleep(Duration::from_millis(1000));
    }
}
