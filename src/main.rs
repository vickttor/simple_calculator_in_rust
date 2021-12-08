use std::io;

fn main() {
    println!("\n --- Simple Calculator ---\n");
    //Creating the main variables
    let mut first_number = String::new();
    let mut second_number = String::new();
    let mut operator_signal = String::new();
    let operation_result: u32;
    //Capturing inputs of the user
    println!("\nFirst Number: ");

    io::stdin()
        .read_line(&mut first_number)
        .expect("[ERROR] It's impossible catch a value. Try later");


    println!("\nOperator Signal: (+ - * / or %) ");
    
    io::stdin()
        .read_line(&mut operator_signal)
        .expect("[ERROR] It's impossible catch the input. Try later");


    println!("\nSecond Number: ");

    io::stdin()
        .read_line(&mut second_number)
        .expect("[ERROR] It's impossible catch a value. Try later");

    //Converting strings to number type
    let first_number: u32 = first_number.trim()
        .parse()
        .expect("It's impossible convert this. Type a Number");

    let second_number: u32 = second_number.trim()
        .parse()
        .expect("It's impossible convert this. Type a Number");


    let operator_signal = operator_signal.trim();

    if operator_signal.len() > 1{
        panic!("\nSelect a valid operator!\n");
    }else if operator_signal == "+" {
        operation_result = first_number + second_number;
    }else if operator_signal == "-"{
        operation_result = first_number - second_number;        
    }else if operator_signal == "*"{
        operation_result = first_number * second_number;
    }else if operator_signal == "/"{
        operation_result = first_number / second_number;
    }else if operator_signal == "%"{
        operation_result = first_number % second_number;
    }else{
        panic!("\nThe operation {} isn't possible here\n", operator_signal);
    }


    println!("\n{} {} {} = {}\n", first_number, operator_signal, second_number, operation_result);

    println!("\n --- Code Finished --- \n");
}
