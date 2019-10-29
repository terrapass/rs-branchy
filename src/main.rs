use std::io::{
    self,
    Read
};

fn main() {
    loop {
        branchy::run();
        io::stdin().read_exact(&mut [0])
            .expect("expected read_exact to succeed");
    }
}
