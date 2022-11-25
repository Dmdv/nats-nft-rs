use futures::executor::block_on;

trait SimpleFuture {
    type Output;
    fn poll(&mut self, wake: fn()) -> Poll<Self::Output>;
}

enum Poll<T> {
    Ready(T),
    Pending,
}

fn main() {
    let future = hello_world();
    block_on(future);
}

async fn hello_world() {
    println!("Hello, world!");
}
