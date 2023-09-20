use std::env;
#[derive(Debug)]
enum Operation {
    Add,
    Subtract,
    Divide,
    Multiply,
}
impl Operation {
    fn apply_operation(&self, num_one: i32, num_two: i32) -> i32 {
        match &self {
            Self::Add => num_one + num_two,
            Self::Subtract => num_one - num_two,
            Self::Divide => num_one / num_two,
            Self::Multiply => num_one * num_two,
        }
    }
    fn from_string(name: &str) -> Self {
        match name {
            "add" => Self::Add,
            "subtract" => Self::Subtract,
            "divide" => Self::Divide,
            "multiply" => Self::Multiply,
            _ => panic!("This is not an operation"),
        }
    }
}
fn main() {
    // env::args().skip(1);
    let args = env::args().skip(1).collect::<Vec<String>>();
    println!("{:?}", args);
    // function that listens to cargo run [anything]
    if args.len() < 3 {
        panic!("Beep boop")
    }
    let input_operation = args.first().unwrap();
    let operation = Operation::from_string(&input_operation);

    // function that takes the operator and the list of numbers

    let numbers = args.iter().skip(1).collect::<Vec<&String>>();
    let num_one = numbers.get(0).unwrap().parse::<i32>().unwrap();
    let num_two = numbers.get(1).unwrap().parse::<i32>().unwrap();

    println!("{:?}", operation);
    println!("{:?}", numbers);
    let output = operation.apply_operation(num_one, num_two);
    println!("{}",output);
    // function that takes all the number from user input and builds an array from all those numbers

    // function figure out what the operator that is in use

    // a(1) + b(2) = c(3)
    // fn add(num1, num2){
    //    return num1  + num2;
    // }
}
