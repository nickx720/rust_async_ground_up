mod asyncindepth;
mod tcp_example;
use asyncindepth::asyncmain;
use tcp_example::tcp_main;

fn main() {
    // tcp_main().expect("something doesn't work");
    asyncmain().expect("something doesn't work");
}
