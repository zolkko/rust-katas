use futures; // 0.1.27
use tokio; // 0.1.21

use futures::prelude::*;
use futures::stream;
use futures::sync::mpsc::unbounded;

fn check() -> impl Future<Item = (), Error = ()> {
    futures::future::lazy(|| {
        let (mut tx, rx) = unbounded();
        let mut rx = rx.and_then(|x| Ok(x));
        assert_eq!(rx.poll().unwrap(), Async::NotReady);
        tx.unbounded_send(1);
        assert_eq!(rx.poll().unwrap(), Async::Ready(Some(1)));
        drop(tx);
        assert_eq!(rx.poll().unwrap(), Async::Ready(None));
        println!("finished");
        Ok(())
    })
}

fn main() {
    tokio::run(check());
}

/*
extern crate futures; // 0.1.27

use futures::prelude::*;
use futures::stream;
use futures::sync::mpsc::unbounded;

use futures::executor::{self, Notify, NotifyHandle};

fn notify_noop() -> NotifyHandle {
    struct Noop;

    impl Notify for Noop {
        fn notify(&self, _id: usize) {}
    }

    const NOOP: &Noop = &Noop;

    NotifyHandle::from(NOOP)
}

fn main() {
    let (mut tx, rx) = unbounded();
    let mut rx = rx.and_then(|x| Ok(x));
    let mut rx = executor::spawn(rx);
    assert_eq!(rx.poll_stream_notify(&notify_noop(), 1).unwrap(), Async::NotReady);
    tx.unbounded_send(1);
    assert_eq!(rx.poll_stream_notify(&notify_noop(), 1).unwrap(), Async::Ready(Some(1)));
    drop(tx);
    assert_eq!(rx.poll_stream_notify(&notify_noop(), 1).unwrap(), Async::Ready(None));
    println!("finished");
}
*/