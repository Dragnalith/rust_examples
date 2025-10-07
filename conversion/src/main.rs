use std::convert::Into;

#[derive(Debug)]
struct Number {
    _value: i32,
}

impl Into<Number> for i32 {
    fn into(self) -> Number {
        Number { _value: self }
    }
}

fn main() {
    let int = 5;
    // Try removing the type annotation
    let num : Number = int.into();
    println!("My number is {:?}", num);
}
