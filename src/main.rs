use tokio::prelude::*;

struct FutureTest {}

impl Future for FutureTest {
    type Item = ();
    type Error = ();
    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        println!("poll called!");
        Ok(Async::Ready(()))
    }
}

fn main() {
    let f = FutureTest {};

    tokio::run(f)
}
