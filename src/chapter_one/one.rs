
pub async fn hello_world() {
    println!("hello world!");
}

pub struct Song {}

pub async fn learn_song() -> Song {
    Song {}
}

pub async fn sing_song(song: Song) {}

pub async fn dance() {}



pub async fn learn_and_sing() {
    let song = learn_song().await;
    sing_song(song).await
}

pub async fn async_main() {
    let f1 = learn_and_sing();
    let f2 = dance();

    futures::join!(f1, f2);
}