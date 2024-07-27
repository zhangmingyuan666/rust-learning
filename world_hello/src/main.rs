fn plus_or_minus(x:i32) -> i32 {
    let mut res=  0;
    if x > 5 {
        res = x - 5
    } else {

    };

    res = x + 5;

    res
}

fn main() {
    let c: i32 = plus_or_minus(1);

    println!("{}", c);
    println!("Hello, world!");
}