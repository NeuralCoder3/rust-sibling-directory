use crate::a::fa1::test_fa1;

pub fn test_fb1() {
    println!("fb1");
    println!("Test call from fb1 to fa1:");
    test_fa1();
}