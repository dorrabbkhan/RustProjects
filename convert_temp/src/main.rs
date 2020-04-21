use std::io;

fn fahr_to_celc(fahr: f64) -> f64 {
    // convert fahrenheit to celcius

    (fahr - 32.0) * 5.0 / 9.0
}

fn celc_to_fahr(celc: f64) -> f64 {
    // convert celcius to fahrenheit

    (celc * 9f64 / 5f64) + 32f64
}

fn main() {

    // print msg's
    println!("Welcome! Please enter: ");
    println!("1 for Fahrenheit to Celcius;");
    println!("2 for Celcius to Fahrenheit.");

    // input user's choice
    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to take input");
    let choice: u8 = choice.trim().parse().expect("Input not a number");

    // check for valid choice
    if (choice != 1) & (choice != 2) {
        println!("ERROR: Invalid choice");
        return
    }
    
    // input user's temperature
    println!("Enter input temperature: ");
    let mut temp = String::new();
    io::stdin()
        .read_line(&mut temp)
        .expect("Failed to take input");
    let temp: f64 = temp.trim().parse().expect("Input not a number");

    // call respective function
    if choice == 1 {

        // get and print result
        let result = fahr_to_celc(temp);
        println!("Result is {} degrees Celcius.", result);
    } else if choice == 2 {

        // get and print result
        let result = celc_to_fahr(temp);
        println!("Result is {} degrees Fahrenheit.", result);
    }
}
