use std::os::unix::prelude::AsRawFd;

use evdev::{Device, EventType};
use mio::{unix::SourceFd, Events, Interest, Poll, Token};

fn main() {
    let mut poll = Poll::new().unwrap();

    let mut devices: Vec<Device> = evdev::enumerate()
        .filter(|d| dbg!(d.supported_events()).contains(EventType::KEY))
        .collect();

    for (i, fd) in devices.iter().enumerate() {
        dbg!(fd.name());
        poll.registry()
            .register(&mut SourceFd(&fd.as_raw_fd()), Token(i), Interest::READABLE)
            .unwrap();
    }

    let mut events = Events::with_capacity(1024);

    loop {
        if poll.poll(&mut events, None).is_err() {
            panic!("Poll failed");
        }

        for mio_event in &events {
            if mio_event.token().0 < devices.len() {
                let idx = mio_event.token().0;
                for ev in devices[idx].fetch_events().unwrap() {
                    dbg!(ev);
                }
            }
        }
    }
}
