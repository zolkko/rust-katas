///! The snippet shows how to use `stream::poll_fn` function
///! and the pinned `sleep` future togather.

use future::Either;
use futures::future::{self, FutureExt, TryFutureExt};
use futures::stream::{self, Stream, StreamExt, TryStreamExt};
use std::error::Error;
use std::io;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;
use tokio::fs::File;
use tokio::time::{sleep, Instant};


async fn make_stream() -> impl Stream<Item = Result<String, io::Error>> + Send {
    const DEFAULT_SLEEP: u64 = 5;

    let file_fut = future::ok::<_, io::Error>(123);
    let stream = file_fut
        .into_stream()
        .map(move |result| {
            let _value = match result {
                Ok(f) => f,
                Err(e) => return Either::Left(stream::once(future::err(e))),
            };

            let mut sleep_fut = Box::pin(sleep(Duration::from_secs(DEFAULT_SLEEP)));
            let mut counter = 0;
            let mut need_reset = true;
            let mapped_result = stream::poll_fn(move |cx| {
                if counter < 10 {
                    if counter == 5 {
                        if need_reset {
                            sleep_fut
                                .as_mut()
                                .reset(Instant::now() + Duration::from_secs(DEFAULT_SLEEP));
                            need_reset = false;
                        }
                        let sleep_poll = sleep_fut.as_mut().poll_unpin(cx);
                        match sleep_poll {
                            Poll::Pending => {
                                println!("pending sleep for {} seconds", DEFAULT_SLEEP);
                                Poll::Pending
                            }
                            Poll::Ready(()) => {
                                println!("sleep ready");
                                counter += 1;
                                Poll::Ready(Some(Ok(format!("sleep number {}", counter))))
                            }
                        }
                    } else {
                        counter += 1;
                        Poll::Ready(Some(Ok(format!("number {}", counter))))
                    }
                } else {
                    counter += 1;
                    Poll::Ready(None)
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
        println!("got next element: {}", res);
    }

    Ok(())
}
