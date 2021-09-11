///! The snippet shows how to use `stream::poll_fn` function
///! and the pinned `sleep` future togather.

use futures::ready;
use futures::future::{self, Either, FutureExt, TryFutureExt};
use futures::stream::{self, Stream, StreamExt, TryStreamExt};
use std::error::Error;
use std::io;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;
use tokio::fs::File;
use tokio::time::{sleep, Instant};
use tokio_util::io::poll_read_buf;
use bytes::{Bytes, BytesMut};

use notify::{Watcher, RecursiveMode, watcher};
use std::sync::mpsc::channel;


async fn make_stream() -> impl Stream<Item = Result<Bytes, io::Error>> + Send {
    const DEFAULT_SLEEP: u64 = 5;
    const BUFF_SIZE: usize = 1024 * 10;

    let file_fut = File::open("/var/log/system.log");
    let stream = file_fut
        .into_stream()
        .map(move |result| {

            
            
            let mut file = match result {
                Ok(f) => f,
                Err(e) => return Either::Left(stream::once(future::err(e))),
            };

            let mut sleep_fut = Box::pin(sleep(Duration::from_secs(DEFAULT_SLEEP)));
            let mut buf = BytesMut::new();
            let mapped_result = stream::poll_fn(move |cx| {
                if buf.capacity() - buf.len() < BUFF_SIZE {
                    buf.reserve(BUFF_SIZE);
                }

                let n = match ready!(poll_read_buf(Pin::new(&mut file), cx, &mut buf)) {
                    Ok(n) => n,
                    Err(err) => {
                        eprintln!("file read error: {}", err);
                        return Poll::Ready(Some(Err(err)));
                    },
                };

                if n == 0 {
                    
                    let mut wait_event = tokio::task::spawn_blocking(move || {
                        println!("watching the file...");
                        let (tx, rx) = channel();
                        let mut watcher = watcher(tx, Duration::from_secs(2)).unwrap();
                        if let Err(e) = watcher.watch("/var/log/system.log", RecursiveMode::Recursive) {
                            eprintln!("failed to watch the file, error: {}", e.to_string());
                            None
                        } else {
                            println!("receiving...");
                            let event = rx.recv();
                            println!("received!!!");
                            Some(event)
                        }
                    });
                    tokio::pin!(wait_event);

                    println!("polling the wait event");
                    match wait_event.as_mut().poll_unpin(cx) {
                        Poll::Pending => Poll::Pending,
                        Poll::Ready(Err(e)) => {
                            eprintln!("failed to join async thread, error: {}", e.to_string());
                            Poll::Pending
                        },
                        Poll::Ready(Ok(_)) => {
                            eprintln!("must be unreachable");
                            Poll::Ready(Some(Ok(Bytes::new())))
                        },
                    }
                } else {
                    let chunk = buf.split().freeze();
                    Poll::Ready(Some(Ok(chunk)))
                }
            });

            Either::Right(mapped_result)
        })
        .flatten();

    stream
}

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    let my_stream = make_stream().await;
    tokio::pin!(my_stream);

    while let Some(item) = my_stream.next().await {
        let res = item?;
        // println!("got next element: {:?}", res);
        // println!("xxx");
    }

    Ok(())
}
