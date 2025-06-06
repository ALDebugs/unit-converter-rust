# CLI Unit Converter
A simple and limited command line unit converter written in Rust. It displays options for conversion and outputs the value in the requested unit.

This was my first Rust project, completed a few weeks ago as an exploration of the Rust language. It served as a valuable learning experience in Rust's syntax, ownership model, and command-line application development.

## Features
* **Inches/Feet/Yards -> Centimeters**
* **Fluid Ounces/Cups/Pints/Quarts/Gallons -> Millimeters**
* **Ounces/Pounds/Stone -> Grams**
* **Fahrenheit <-> Celsius**

## Usage
* Make sure you have Rust installed: [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)

1.  **Clone the repository:**
    ```bash
    git clone [https://github.com/ALDebugs/unit-converter-rust.git](https://github.com/ALDebugs/unit-converter-rust.git)
    cd unit-converter-rust
    ```
2.  **Run the application:**
    ```bash
    cargo run
    ```
    The program will then guide you through the available conversion options.

### Example
```bash
Hello, what would you like to convert?
0) Distance
1) Volume
2) Weight
3) Temperature
Enter 'exit' to quit.
0
Distance to Centimeters
Please select: 
        0) Inches, 
        1) Feet, 
        2) Yards
1
How many Feet do you want to convert to centimeters?
7
7 feet is 213.36 centimeters
```

## To Do
* Expand input and output options
* Improve readability of the terminal
* Ask if the user has any more unit conversion requests before shutting down
* Refactor code for clarity
