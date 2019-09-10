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

fn main() {
    let future = hello_world();
    block_on(future);

    let future = async_main();
    block_on(future);
}
