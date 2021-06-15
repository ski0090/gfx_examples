extern crate hello_macro;
use hello_macro::make_answer;

make_answer!();
fn main() {
    println!("{}", answer(3));
}

