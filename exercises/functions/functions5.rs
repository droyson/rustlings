// functions5.rs
// Make me compile! Execute `rustlings hint functions5` for hints :)

fn main() {
    let answer = square(12345);
    println!("The answer is {}", answer);
}

fn square(num: i32) -> i128 {
    return (num * num) as i128;
}
