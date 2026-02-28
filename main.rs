	use std::io;

// Create in me a pure heart, O God,
// and renew a steadfast spirit within me.
// Psalm 51:10

fn add() {

    //First Num
    let mut num1 = String::new();
    println!("Please enter the First number you want to input: ");
    io::stdin().read_line(&mut num1).expect("Failed to read line");
    let num1f = num1.trim().parse::<f64>().unwrap(); //converts string to f64


    // Second Num
    let mut num2 = String::new();
    println!("Please enter the Second number you want to input: ");
    io::stdin().read_line(&mut num2).expect("Failed to read line");
    let num2f = num2.trim().parse::<f64>().unwrap(); //converts string to f64


    let answer = num1f + num2f;
    println!("{} + {} = {}", num1f, num2f, answer);

}

fn sub() {

    //First Num
    let mut num1 = String::new();
    println!("Please enter the First number you want to input: ");
    io::stdin().read_line(&mut num1).expect("Failed to read line");
    let num1f = num1.trim().parse::<f64>().unwrap(); //converts string to f64


    // Second Num
    let mut num2 = String::new();
    println!("Please enter the Second number you want to input: ");
    io::stdin().read_line(&mut num2).expect("Failed to read line");
    let num2f = num2.trim().parse::<f64>().unwrap(); //converts string to f64


    let answer = num1f - num2f;
    println!("{} - {} = {}", num1f, num2f, answer);


}

fn multi() {

    //First Num
    let mut num1 = String::new();
    println!("Please enter the First number you want to input: ");
    io::stdin().read_line(&mut num1).expect("Failed to read line");
    let num1f = num1.trim().parse::<f64>().unwrap(); //converts string to f64


    // Second Num
    let mut num2 = String::new();
    println!("Please enter the Second number you want to input: ");
    io::stdin().read_line(&mut num2).expect("Failed to read line");
    let num2f = num2.trim().parse::<f64>().unwrap(); //converts string to f64


    let answer = num1f * num2f;
    println!("{} * {} = {}", num1f, num2f, answer);


}


fn div() {

    //First Num
    let mut num1 = String::new();
    println!("Please enter the First number you want to input: ");
    io::stdin().read_line(&mut num1).expect("Failed to read line");
    let num1f = num1.trim().parse::<f64>().unwrap(); //converts string to f64


    // Second Num
    let mut num2 = String::new();
    println!("Please enter the Second number you want to input: ");
    io::stdin().read_line(&mut num2).expect("Failed to read line");
    let num2f = num2.trim().parse::<f64>().unwrap(); //converts string to f64

    if num2f == 0.0 {
        println!("did bro forget that u cant divide by 0 💀");
        return;
    }

    let answer = num1f / num2f;
    println!("{} / {} = {}", num1f, num2f, answer);


}


fn square() {

    let mut num1 = String::new();
    println!("Number to square: ");
    io::stdin().read_line(&mut num1).expect("Failed to read line");
    let num1f = num1.trim().parse::<f64>().unwrap(); //converts string to f64


    let answer = num1f * num1f;
    println!("{} squared is {}", num1f, answer);

}

fn sqrt() {

    let mut num1 = String::new();
    println!("Number to square root: ");
    io::stdin().read_line(&mut num1).expect("Failed to read line");
    let num1f = num1.trim().parse::<f64>().unwrap(); //converts string to f64


    let answer = num1f.sqrt();
    println!("{} square root is {}", num1f, answer);

}


fn main() {
    loop {
    println!("Choose which one you want to do");
    println!("Add (1)");
    println!("Subtract (2)");
    println!("Multiply (3)");
    println!("Divide (4)");
    println!("Square (5)");
    println!("Square Root (6)");
    println!("Exit (x)");
    let mut operation = String::new();
    println!("Enter your choice: ");
    io::stdin().read_line(&mut operation).expect("Failed to read line");

    match operation.trim() {
        "1" => add(),
        "2" => sub(),
        "3" => multi(),
        "4" => div(),
        "5" => square(),
        "6" => sqrt(),
        "x" => break,
        _ => println!("Invalid choice! Try again."),
    };
    }

}
