use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};
use std::thread;
use std::time::Duration;

use futures::executor::block_on;


async fn hello() {
    print!("hello");
}

async fn space() {
    print!(", ");
}

async fn world() {
    println!("world!");
}

async fn hello_world() {
    hello().await;
    space().await;
    world().await;
}

struct Song(String);

async fn learn_song() -> Song {
    Song("Test song".to_owned())
}

async fn sing_song(song: Song) {
    println!("Singing the song {}", song.0);
}

async fn dance() {
    println!("Dancing the song");
}

async fn learn_and_sing() {
    let song = learn_song().await;
    sing_song(song).await;
}

async fn async_main() {
    let f1 = learn_and_sing();
    let f2 = dance();
    futures::join!(f1, f2);
}

struct SharedState {
    completed: bool,
    waker: Option<Waker>,
}

pub struct TimerFuture {
    shared_state: Arc<Mutex<SharedState>>,
}

impl Future for TimerFuture {
    type Output = ();
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut shared_state = self.shared_state.lock().unwrap();
        if shared_state.completed {
            Poll::Ready(())
        } else {
            shared_state.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

fn timer(duration: Duration) -> TimerFuture {
    let mut state = Arc::new(Mutex::new(SharedState { completed: false, waker: None }));
    let mut thread_state = Arc::clone(&state);

    thread::spawn(move || {
        thread::sleep(duration);

        let mut ss = thread_state.lock().unwrap();
        ss.completed = true;
        if let Some(waker) = ss.waker.take() {
            waker.wake();
        }
    });

    TimerFuture { shared_state: state, }
}

async fn timer_test() {
    println!("before timer");
    timer(Duration::from_secs(5)).await;
    println!("after timer");
}

fn main() {
    let future = hello_world();
    block_on(future);

    let future = async_main();
    block_on(future);

    let future = timer_test();
    block_on(future);
}
