use std::io;


#[derive(Debug)]
enum Solution  {
    Addition{vector: Vec<u32>},
    Substraction{vector: Vec<u32>},
    Division{vector: Vec<f64>},
    Multiply{vector: Vec<f64>},
}

impl Solution{
    fn equation(&self) {
        match &self {
            Solution::Addition{vector: numbers} => 
                for digit in numbers.iter() {
                    while digit != numbers.iter().last().unwrap() {
                        2 + 2;

                        if digit == numbers.iter().last().unwrap() {
                            break;
                        }
                    }
                },
            Solution::Substraction{vector: numbers} => 
                for digit in numbers.iter() {
                    while digit != numbers.iter().last().unwrap() {
                        digit - digit;

                        if digit == numbers.iter().last().unwrap() {
                            break;
                        }
                    }
                },
            Solution::Division{vector: numbers} => 
                for digit in numbers.iter() {
                    while digit != numbers.iter().last().unwrap() {
                        digit / digit;
                        
                        if digit == numbers.iter().last().unwrap() {
                            break;
                        }
                    }
                },
            Solution::Multiply{vector: numbers} => 
                for digit in numbers.iter() {
                    while digit != numbers.iter().last().unwrap() {
                        digit * digit;

                        if digit == numbers.iter().last().unwrap() {
                            break;
                        }
                    }
                },
        }
    }
}

fn main() {
    println!("Input your numbers, Type 'End' to proceed calculation");

    let mut numbers: Vec<u32> = Vec::new();

    loop {
        let mut number_input = String::new();

        io::stdin().read_line(&mut number_input)
            .expect("Failed to read line");

        let number_input: String = number_input.trim().parse()
            .expect("Please type a numbers!");

        if number_input == "End" {
            break;
        }

        let number_input: u32 = number_input.trim().parse()
            .expect("Please type a numbers!");

        numbers.push(number_input);
    }

    // println!("numbers are {:?}", numbers);
    println!("Input your sign");

    let mut sign = String::new();

    io::stdin().read_line(&mut sign)
        .expect("Failed to read line");

    let sign: String = sign.trim().parse()
        .expect("Please type a sign!");

    let sign = if sign == "+" {
        Solution::Addition{vector: numbers}
        } else if sign == "/" {
        let new_num1: Vec<f64> = numbers.into();
        Solution::Division{vector: new_num1}
        } else if sign == "*" {
        let new_num1: Vec<f64> = numbers.into();
        Solution::Multiply{vector: new_num1}
        } else {
        Solution::Substraction{vector: numbers}
        };
    
    let answer_question = sign.equation();
    // match answer_question {
    //     Answer::Integer {numbers} => println!("Your Answer is {}", numbers), 
    //     Answer::Float {numbers} => println!("Your Answer is {}", numbers),
    // }
}
