use std::io;

fn main() {
    println!("FC Temp Convertor");
    println!("-------୨◥▶ل͜◀◤୧-------");

    println!("What scale would you like to convert to?");

    let mut scale = String::new();
    io::stdin().read_line(&mut scale).expect("Failed to read line");
    scale.make_ascii_lowercase();

    println!("Enter your value:");
    let mut value = String::new();
    io::stdin().read_line(&mut value).expect("Failed to read line");
    let value: i32 = match value.trim().parse() {
        Ok(value) => value,
        Err(_) => {
            println!("Couldn't parse string to int");
            return;
        }
    };

    if scale.contains("c") {
        println!("{} F  = {} C", value, ((value-32)*5)/9);
    } else if scale.contains("f") {
        println!("{} C  = {} F", value, ((value/5)*9)+32);
    } else {
        println!("Sorry :(, can only convert between Celsius and Fahrenheit");
    }
}
