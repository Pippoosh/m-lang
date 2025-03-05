## M Programming Language

M is a custom programming language created as a tribute to Misty, a beloved cat whose memory lives on through this project. Though Misty is no longer with us, her playful curiosity and elegant grace continue to inspire this language. The M language aims to provide an intuitive and expressive programming experience that honors her spirit.

## In Memory of Misty

Misty is the inspiration behind this programming language. With her sharp intellect, playful nature, and ability to transform any ordinary moment into something magical, she embodied the core philosophy of the M language: simplicity, expressiveness, and transformation.

The 'M' in M language stands for Misty, honoring her memory and the lasting impact she had. Her curious nature and problem-solving abilities inspired many of the language's features. Though she wasn't physically present during the development of this project, her spirit guided its creation.

## Features

*   **Variables**: Define and use variables of different types (numbers, strings, arrays)
*   **Functions**: Define and call functions with parameters
*   **Loops**: Use `for` and `while` loops for iteration
*   **Conditionals**: Use `if`, `else if`, and `else` statements for conditional logic
*   **Transformers**: Define and use transformers, which are applied to values using dot notation
*   **Transformer Chaining**: Chain transformer calls using dot notation (e.g., `x.a.b.c()`)
*   **Standard Library**: A comprehensive standard library with math, string, and array utilities
*   **Input Function**: Interactive input capabilities to gather user input during program execution
*   **Type Conversion**: Built-in transformers for converting between different data types

## Getting Started

### Prerequisites

*   Rust (for building the interpreter)

### Building

```plaintext
cargo build
```

### Running

```plaintext
cargo run [file_path]
```

If no file path is provided, the interpreter will run the default `main.m` file.

## Examples

### Basic Example

```plaintext
// Define variables
x = 10
y = 20

// Define a function
fn add(a, b) {
    return a + b
}

// Call the function
result = add(x, y)
print("The result is: " + result)
```

### Transformer Example

```plaintext
// Define transformers
transformer square() {
    return applied * applied
}

transformer add(n) {
    return applied + n
}

// Use transformers with chaining
x = 5
result = x.square().add(10)
print("The result is: " + result)  // Output: The result is: 35
```

### Using the Standard Library

```plaintext
// Import the standard library
use "stdlib/core.m"

// Use math functions and transformers
num = 5
print(num + " squared is " + num.square())
print("The factorial of " + num + " is " + factorial(num))

// Use string functions and transformers
greeting = "Hello, World!"
print("Original: " + greeting)
print("Reversed: " + greeting.reverse())
print("Length: " + greeting.length())

// Use array functions and transformers
numbers = [1, 2, 3, 4, 5]
print("Sum: " + numbers.sum())
print("Average: " + numbers.average())
```

### Interactive Input Example

```plaintext
// Import the standard library
use "stdlib/core.m"

// Get user input
name = input("What is your name? ")
print("Hello, " + name + "!")

// Get age and convert to number
age_str = input("How old are you? ")
if age_str.parse_number().to_string() == age_str {
    age = age_str.to_number()
    print("In 10 years, you will be " + (age + 10) + " years old.")
} else {
    print("That's not a valid age!")
}
```

## Standard Library

M comes with a standard library that provides common functions and transformers for math, string, and array operations. See the [Standard Library README](stdlib/README.md) for more information.

## Language Syntax

### Variables

```plaintext
name = "John"
age = 30
numbers = [1, 2, 3, 4, 5]
```

### Functions

```plaintext
fn add(a, b) {
    return a + b
}

fn is_even(n) {
    return (n % 2) == 0
}
```

### Transformers

```plaintext
transformer square() {
    return applied * applied
}

transformer add(n) {
    return applied + n
}
```

### Loops

```plaintext
// For loop
for i in range(0, 10) {
    print(i)
}

// While loop
i = 0
while i &lt; 10 {
    print(i)
    i = i + 1
}

// For-each loop
arr = [1, 2, 3, 4, 5]
for item in arr {
    print(item)
}
```

### Conditionals

```plaintext
if x &gt; 10 {
    print("x is greater than 10")
} else if x &gt; 5 {
    print("x is greater than 5 but not greater than 10")
} else {
    print("x is not greater than 5")
}
```

### Importing Files

```plaintext
use "path/to/file.m"
```

## Type Conversion Transformers

The M language includes built-in transformers for converting between different data types:

```plaintext
// Convert to string
num = 42
str_num = num.to_string()  // "42"

// Convert to number
str = "42"
num = str.to_number()  // 42

// Convert to boolean
zero = 0
bool_zero = zero.to_bool()  // false

// Convert to array
value = 42
arr = value.to_array()  // [42]

// Parse string to number
str = "123.45"
num = str.parse_number()  // 123.45

// Parse string to boolean
str = "true"
bool_val = str.parse_bool()  // true

// Convert to JSON
arr = [1, "two", true]
json = arr.to_json()  // "[1,\"two\",true]"
```

## License

This project is open source and available under the MIT License.

## Acknowledgments

*   Special thanks to Misty, whose memory continues to inspire this project
*   The Rust programming language community for providing the foundation for the M interpreter
*   All cat lovers and programmers who understand that the best code is written with a cat nearby