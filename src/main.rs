use std::io;
use colored::*;

const INCH_TO_CM: f64 = 2.54;
const FEET_TO_CM: f64 = 30.48;
const YARD_TO_CM: f64 = 91.44;

const FLUID_OUNCE_TO_ML: f64 = 29.5735;
const CUP_TO_ML: f64 = 236.588;
const PINT_TO_ML: f64 = 473.176;
const QUART_TO_ML: f64 = 946.353;
const GALLON_TO_ML: f64 = 3785.41;

const OUNCE_TO_GRAMS: f64 = 28.3495;
const POUND_TO_GRAMS: f64 = 453.592;
const STONE_TO_GRAMS: f64 = 6350.29;

const FAHRENHEIT_TO_CELSIUS: f64 = -17.222222;
const CELSIUS_TO_FAHRENHEIT: f64 = 33.8;

fn main() {
    
    loop {
        println!("Hello, what would you like to convert?");
    
        println!("0) Distance");
        println!("1) Volume");
        println!("2) Weight");
        println!("3) Temperature");
        println!("Enter 'exit' to quit.");
        
        let mut choice_main = String::new();
        let mut choice_two: i32;
    
        io::stdin()
            .read_line(&mut choice_main)
            .expect("Failed to read line");
    
        if choice_main.to_lowercase().contains("exit") {
            println!("Exiting...");
            return;
        }
        let choice_main: u32 = match choice_main.trim().parse() {
            Ok(choice) => choice,
            Err(_) => {
                println!("{}", "Invalid choice".red());
                continue;
            }
        };
        match choice_main {
            0 => {
                println!("Distance to Centimeters");
                'centimeters: loop {
                    
                    println!("Please select: \n\t0) Inches, \n\t1) Feet, \n\t2) Yards");
                    choice_two = input_parse();
                    if choice_two < 0 || choice_two > 2 {
                        println!("Invalid choice");
                        continue 'centimeters;
                    }
                    let choice_name = match choice_two {
                        0 => "Inches",
                        1 => "Feet",
                        2 => "Yards",
                        _ => "",
                    };
                    println!("How many {} do you want to convert to {}?", choice_name.purple(), "centimeters".purple());
                    let input_value = input_value_parse();
                    let result;
                    match choice_two {
                        0 => {
                            result = input_value * INCH_TO_CM;
                            println!("{} inches is {} centimeters", input_value, result);
                            return;
                        }
                        1 => {
                            result = input_value * FEET_TO_CM;
                            println!("{} feet is {} centimeters", input_value, result);
                            return;
                        }
                        2 => {
                            result = input_value * YARD_TO_CM;
                            println!("{} yards is {} centimeters", input_value, result);
                            return;
                        }
                        _ => {
                            println!("Invalid choice");
                            continue 'centimeters;
                        }
                    }
                }
            },
            1 => {
                println!("Volume to Milliliters");
                'milliliters: loop {
                    println!("Please select: \n\t0) Ounces, \n\t1) Cups, \n\t2) Pints, \n\t3) Quarts, \n\t4) Gallons");
                    choice_two = input_parse();
                    if choice_two < 0 || choice_two > 4 {
                        println!("Invalid choice");
                        continue 'milliliters;
                    }
                    let choice_name = match choice_two {
                        0 => "Ounces",
                        1 => "Cups",
                        2 => "Pints",
                        3 => "Quarts",
                        4 => "Gallons",
                        _ => "",
                    };
                    println!("How many {} do you want to convert to {}?", choice_name.purple(), "milliliters".purple());
                    let input_value = input_value_parse();
                    let result;
                    
                    match choice_two {
                        0 => {
                            result = input_value * FLUID_OUNCE_TO_ML;
                            println!("{} ounces is {} milliliters", input_value, result);
                            return;
                        }
                        1 => {
                            result = input_value * CUP_TO_ML;
                            println!("{} cups is {} milliliters", input_value, result);
                            return;
                        }
                        2 => {
                            result = input_value * PINT_TO_ML;
                            println!("{} pints is {} milliliters", input_value, result);
                            return;
                        }
                        3 => {
                            result = input_value * QUART_TO_ML;
                            println!("{} quarts is {} milliliters", input_value, result);
                            return;
                        }
                        4 => {
                            result = input_value * GALLON_TO_ML;
                            println!("{} gallons is {} milliliters", input_value, result);
                            return;
                        }
                        _ => {
                            println!("Invalid choice");
                            continue 'milliliters;
                        }
                    }
                    
                }
            },
            2 => {
                println!("Weight to Grams");
                'grams: loop {
                    
                    println!("Please select: \n\t0) Ounces, \n\t1) Pounds, \n\t2) Stones");
                    choice_two = input_parse();
                    if choice_two < 0 || choice_two > 1 {
                        println!("Invalid choice");
                        continue 'grams;
                    }
                    let choice_name = match choice_two {
                        0 => "Ounces",
                        1 => "Pounds",
                        2 => "Stones",
                        _ => "",
                    };
                    println!("How many {} do you want to convert to {}?", choice_name.purple(), "grams".purple());
                    let input_value = input_value_parse();
                    let result;
                    match choice_two {
                        0 => {
                            result = input_value * OUNCE_TO_GRAMS;
                            println!("{} ounces is {} grams", input_value, result);
                            return;
                        }
                        1 => {
                            result = input_value * POUND_TO_GRAMS;
                            println!("{} pounds is {} grams", input_value, result);
                            return;
                        }
                        2 => {
                            result = input_value * STONE_TO_GRAMS;
                            println!("{} stones is {} grams", input_value, result);
                            return;
                        }
                        _ => {
                            println!("Invalid choice");
                            continue 'grams;
                        }
                    }
                }

            },
            3 => {
                println!("Temperature");
                'temperature: loop {
                    println!("Please select: \n\t0) °F -> °C, \n\t1) °C -> °F ");
                    choice_two = input_parse();
                    if choice_two < 0 || choice_two > 1 {
                        println!("{}", "Invalid choice".red());
                        continue 'temperature;
                    }
                    let choice_name = match choice_two {
                        0 => "Fahrenheit",
                        1 => "Celsius",
                        _ => "",
                    };
                    let conv_choice = if choice_two == 1 {"Fahrenheit"} else {"Celsius"};
                    println!("How many {} do you want to convert to {}?", choice_name.purple(), conv_choice.purple());
                    let input_value = input_value_parse();
                    let result;
                    match choice_two {
                        0 => {
                            result = input_value * FAHRENHEIT_TO_CELSIUS;
                            println!("{}°F is {}°C", input_value, result);
                            return;
                        }
                        1 => {
                            result = input_value * CELSIUS_TO_FAHRENHEIT;
                            println!("{}°C is {}°F", input_value, result);
                            return;
                        }
                        _ => {
                            println!("Invalid choice");
                            continue 'temperature;
                        }
                    }
                }
            },
            _ => {
                println!("{}", "Invalid choice".red());
                continue;
            }
        }
    }
}

fn input_parse() -> i32 {
    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");

    match choice.trim().parse() {
        Ok(number) => number,
        Err(_) => {
            println!("{}", "Invalid choice".red());
            -1
        }
    }
    
}
fn input_value_parse() -> f64 {
    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect(&"Failed to read line".red());

    match choice.trim().parse() {
        Ok(number) => number,
        Err(_) => {
            println!("{}", "Invalid choice".red());
            -1.0
        }
    }
    
}

