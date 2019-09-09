#![feature(nightly, async_await, futures_api, generators)]

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

//struct Song(String);
//
//async fn learn_song() -> Song {
//    Song("the song".to_owned())
//}
//
//async fn sing_song(song: Song) {
//    println!("singing the song {}", song.0);
//}
//
//async fn dance() {
//    println!("dancing the song");
//}
//
//async fn learn_and_sing() {
//    let song = learn_song().await;
//    sing_song(song).await;
//}
//
//async fn async_main() {
//    let f1 = learn_and_sing();
//    let f2 = dance();
//
//    futures::join!(f1, f2);
//}

fn main() {
    let future = hello_world();
    block_on(future);
}
