// core.m - Core standard library that imports all other modules

// Import all standard library modules
use "stdlib/math.m"
use "stdlib/string.m"
use "stdlib/array.m"

// Additional core functions and transformers

// Type checking functions to be implemented in interpreter
// fn is_number(value) {
//     return value == value + 0
// }
// 
// fn is_string(value) {
//     return value == value + ""
// }
//
// fn is_array(value) {
//     return value == value
// }
// fn is_function(value) {
//     return value == value
// }
//

// Print array function

fn print_array(arr) {
    print("[")
    for item in arr {
        print(item)
        print(",")
    }
    print("]")
}

// Input function - this will be implemented in the interpreter
fn input(prompt) {
    // This function will be handled by the interpreter
    // It should display the prompt and return user input as a string
    return ""
}

// Print a message to show the core library was loaded
print("Core library loaded successfully")
