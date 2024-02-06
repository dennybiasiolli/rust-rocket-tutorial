use concurrency::channels;
use concurrency::mutex;
use concurrency::threads;

fn main() {
    threads::test1();
    threads::test2();

    channels::test1();

    mutex::test1();
    mutex::test2();
}
