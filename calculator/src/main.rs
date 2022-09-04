use std::env::{args, Args};

fn main() {
    let mut args: Args = args();

    let first = args.nth(1).unwrap();
    let operator = args.nth(0).unwrap().chars().next().unwrap();
    let second = args.nth(0).unwrap();

    let first: f32 = first.parse().unwrap();
    let second: f32 = second.parse().unwrap();

    let result = operate(operator, first, second);

    println!("{}", output(first, operator, second, result));
}

fn operate(operator: char, first: f32, second: f32) -> f32 {
    match operator {
        '+' => first + second,
        '-' => first - second,
        '*' | 'x' | 'X' => first * second,
        '/' => first / second,
        _ => panic!("Invalid operator used!")
    }
}

fn output(first: f32, operator: char, second: f32, result: f32) -> String {
    format!("{} {} {} = {}", first, operator, second, result)
}
