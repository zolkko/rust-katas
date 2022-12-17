use std::fmt;
use std::task::{Poll, Context};
use std::future::Future;
use std::pin::Pin;
use futures::{pin_mut, StreamExt, Stream};
use futures::stream::{repeat, once};
use tokio::time::{sleep, Instant, Sleep};
use tokio::time::Duration;
use futures::ready;
use std::mem;
use pin_project_lite::pin_project;


pin_project! {
    pub(crate) struct TimedBufferedStream<St> where St: Stream {
        #[pin]
        stream: St,
        #[pin]
        deadline: Sleep,
        duration: Duration,
        poll_deadline: bool,
        capacity: usize,
        buffer: Vec<St::Item>,
    }
}

impl<St: Stream> TimedBufferedStream<St> {
    fn new(stream: St, capacity: usize, duration: Duration) -> Self {
        let next = Instant::now() + duration;
        let deadline = tokio::time::sleep_until(next);

        Self {
            stream,
            deadline,
            duration,
            poll_deadline: true,
            capacity,
            buffer: Vec::with_capacity(capacity),
        }

    }
}

impl<St> Stream for TimedBufferedStream<St>
where
    St: Stream,
    <St as Stream>::Item: fmt::Debug,
{
    type Item = Vec<St::Item>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if self.buffer.capacity() == 0 {
            return Poll::Ready(None);
        }

        let mut this = self.project();

        loop {
            let item = this.stream.as_mut().poll_next(cx);
            match item {
                Poll::Ready(Some(value)) => {
                    let next = Instant::now() + *this.duration;
                    this.deadline.as_mut().reset(next);
                    *this.poll_deadline = true;

                    this.buffer.push(value);
                    if this.buffer.len() == this.buffer.capacity() {
                        let result: Vec<St::Item> = mem::replace(this.buffer, Vec::with_capacity(*this.capacity));
                        return Poll::Ready(Some(result));
                    } else {
                        // spin the loop as there is another element...
                    }
                },
                Poll::Ready(None) => {
                    return if this.buffer.is_empty() {
                        Poll::Ready(None)
                    } else {
                        let result: Vec<St::Item> = mem::take(this.buffer);
                        Poll::Ready(Some(result))
                    };
                },
                Poll::Pending => {
                    return if *this.poll_deadline {
                        ready!(this.deadline.as_mut().poll(cx));
                        *this.poll_deadline = false;

                        if this.buffer.is_empty() {
                            Poll::Pending
                        } else {
                            let result: Vec<St::Item> = mem::replace(this.buffer, Vec::with_capacity(*this.capacity));
                            Poll::Ready(Some(result))
                        }
                    } else {
                        Poll::Pending
                    };
                },
            }
        }
        // end of the function
    }
}

impl<St> fmt::Debug for TimedBufferedStream<St>
    where
        St: fmt::Debug + Stream,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TimedBufferedStream").field("stream", &self.stream).finish()
    }
}


#[tokio::main]
async fn main() {
    let stream1 = repeat(1).take(5);
    let stream2 = once(async {
        sleep(Duration::from_millis(1000)).await;
        3
    });
    let stream3 = repeat(2).take(5);

    let stream = stream1.chain(stream2).chain(stream3);
    pin_mut!(stream);

    let st = TimedBufferedStream::new(stream, 2, Duration::from_millis(10));
    let x = st.collect::<Vec<_>>().await;
    println!("x = {x:?}");
}
