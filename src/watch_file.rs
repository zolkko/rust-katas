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
                    sleep_fut.as_mut().reset(Instant::now() + Duration::from_secs(DEFAULT_SLEEP));
                    match sleep_fut.as_mut().poll_unpin(cx) {
                        Poll::Pending => Poll::Pending,
                        Poll::Ready(()) => {
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
        println!("got next element: {:?}", res);
    }

    Ok(())
}
