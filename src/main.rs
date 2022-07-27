mod asyncindepth;
mod tcp_example;
mod webserver;
use asyncindepth::asyncmain;
use tcp_example::tcp_main;
use webserver::webservermain;

fn main() {
    // tcp_main().expect("something doesn't work");
    // asyncmain().expect("something doesn't work");
    webservermain().expect("something doesn't work");
}
