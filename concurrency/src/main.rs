use concurrency::threads;
use concurrency::channels;

fn main() {
    threads::test1();
    threads::test2();

    channels::test1();
}
