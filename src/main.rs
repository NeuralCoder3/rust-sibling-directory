// via lib.rs
// use sibling::b::fb1::test_fb1;

// via main.rs
mod a;
mod b;

use b::fb1::test_fb1;

fn main() {
    test_fb1();
}
