

// how to convert f to c in rust 

// Step 1: Import any necessary modules (println! is in prelude, so no imports needed)

// Step 2: Define a function called fahrenheit_to_celsius that takes an f64 parameter



// Step 3: Inside the function, apply the conversion formula: (fahrenheit - 32.0) * 5.0 / 9.0

// Step 4: Return the calculated Celsius value

// Step 5: Create a main function

// Step 6: Define a variable to hold a Fahrenheit temperature value

// Step 7: Call the conversion function and store the result

// Step 8: Print the result using println! macro with formatting

// Step 9: Optionally add more test cases with different temperatures

// Step 10: Compile with: cargo build or rustc filename.rs

// Step 11: Run with: cargo run or ./filename


pub fn fahrenheit_to_celsius(temp_in_f:f64)-> f64 {
    let  temp_in_c:f64 = (temp_in_f - 32.0) * 5.0 / 9.0;
    return temp_in_c;
}

fn main(){
    let result:f64 = fahrenheit_to_celsius(50.0);
    println!("This is temp in celcius {:}", result);
}