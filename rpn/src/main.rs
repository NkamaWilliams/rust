fn main() {
    println!("Hello, world!");
    let array = ["2", "1", "+", "3", "*"];
    let vector = array.map(|elem| String::from(elem)).to_vec();
    println!("{}", eval_rpn(vector));
}

fn eval_rpn(tokens: Vec<String>) -> i64 {
    let mut stack = vec![];
    let mut a: i64;
    let mut b: i64;
    for token in tokens {
        match token.as_str() {
            "+" => {
                b = stack.pop().unwrap();
                a = stack.pop().unwrap();
                stack.push(a + b);
            }
            "-" => {
                b = stack.pop().unwrap();
                a = stack.pop().unwrap();
                stack.push(a - b);
            }
            "*" => {
                b = stack.pop().unwrap();
                a = stack.pop().unwrap();
                stack.push(a * b);
            }
            "/" => {
                b = stack.pop().unwrap();
                a = stack.pop().unwrap();
                stack.push(a / b);
            }
            _ => {
                stack.push(token.parse().unwrap());
            }
        }
    }
    stack.pop().unwrap()
}
