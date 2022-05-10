use std::env::{args, Args};

fn main() {
    let mut args: Args = args();

    let operand_one = args.nth(1).unwrap();
    let operator = args.nth(0).unwrap().chars().next().unwrap();
    let operand_two = args.nth(0).unwrap();

    let one_parsed = operand_one.parse::<f32>().unwrap();
    let second_parsed: f32 = operand_two.parse::<f32>().unwrap();

    let result = operate(one_parsed,operator,second_parsed);

    println!("{:?}", output(one_parsed,operator,second_parsed,result));
}

fn operate(first: f32, operator: char, second: f32) -> f32{
    match operator {
        '+' => first + second,
        '-' => first - second,
        '/' => first / second,
        '*' | 'x' => first * second,
        '^' => first.powf(second),
            _ => panic!("Please use valid arguments.")
    }
}

fn output(first: f32, operator: char, second: f32, result: f32) -> String{
    return format!("Calculation: {} {} {} = {}", first, operator, second, result);
}