fn main() {
    collatz_sequence(500);
}

//Function for Collatz Sequence
fn collatz_sequence (mut x: i32) -> i32 {
    while x != 1{
        if x == 1 {
            println!("{}", x);
        } else if x % 2 == 0 {
            x = x / 2;
            println!("{}", x);
        } else {
            x = (x * 3) + 1;
            println!("{}", x);
        }
    }
    return 0
}
