# M Language Standard Library

This directory contains the standard library for the M programming language. The standard library provides a set of common functions and transformers that can be used in your M programs.

## How to Use

To use the standard library in your M programs, simply import the core module:

```
use "stdlib/core.m"
```

This will import all the standard library modules, including math, string, and array utilities.

## Available Modules

### Core Module (`core.m`)

The core module imports all other standard library modules and provides additional utility functions:

- Type checking functions: `is_number()`, `is_string()`, `is_array()`, `is_function()`
- Print functions: `println()`, `print_array()`

### Math Module (`math.m`)

The math module provides mathematical functions and transformers:

**Functions:**
- `abs(x)`: Returns the absolute value of x
- `max(a, b)`: Returns the maximum of a and b
- `min(a, b)`: Returns the minimum of a and b
- `pow(base, exponent)`: Returns base raised to the power of exponent
- `factorial(n)`: Returns the factorial of n
- `is_even(n)`: Returns true if n is even
- `is_odd(n)`: Returns true if n is odd

**Transformers:**
- `abs()`: Returns the absolute value of the applied value
- `square()`: Returns the square of the applied value
- `cube()`: Returns the cube of the applied value
- `sqrt()`: Returns the square root of the applied value
- `negate()`: Returns the negation of the applied value
- `increment()`: Returns the applied value plus 1
- `decrement()`: Returns the applied value minus 1

### String Module (`string.m`)

The string module provides string manipulation functions and transformers:

**Functions:**
- `concat(a, b)`: Concatenates two strings
- `repeat(str, times)`: Repeats a string a specified number of times

**Transformers:**
- `length()`: Returns the length of the applied string
- `uppercase()`: Returns the uppercase version of the applied string
- `lowercase()`: Returns the lowercase version of the applied string
- `reverse()`: Returns the reversed version of the applied string
- `trim()`: Returns the applied string with whitespace trimmed from both ends

### Array Module (`array.m`)

The array module provides array manipulation functions and transformers:

**Functions:**
- `create_array(size, default_value)`: Creates an array of the specified size with the default value
- `array_get(arr, index)`: Gets the value at the specified index in the array
- `array_set(arr, index, value)`: Returns a new array with the value at the specified index changed

**Transformers:**
- `length()`: Returns the length of the applied array
- `sum()`: Returns the sum of all elements in the applied array
- `average()`: Returns the average of all elements in the applied array
- `map(func)`: Applies a function to each element in the array and returns a new array
- `filter(predicate)`: Returns a new array with only the elements that satisfy the predicate
- `reverse()`: Returns a new array with the elements in reverse order
- `sort()`: Returns a new array with the elements sorted in ascending order

## Examples

Here's an example of using the standard library:

```
// Import the standard library
use "stdlib/core.m"

// Math examples
num = 5
print(num + " squared is " + num.square())
print("The factorial of " + num + " is " + factorial(num))

// String examples
greeting = "Hello, World!"
print("Original: " + greeting)
print("Reversed: " + greeting.reverse())
print("Length: " + greeting.length())

// Array examples
numbers = [1, 2, 3, 4, 5]
print("Array: ")
print_array(numbers)
print("Sum: " + numbers.sum())
print("Average: " + numbers.average())

// Filter for even numbers
even_numbers = numbers.filter(is_even)
print("Even numbers: ")
print_array(even_numbers)

// Map to double each number
doubled = numbers.map(double)
print("Doubled: ")
print_array(doubled)
```

## Extending the Standard Library

You can extend the standard library by adding new functions and transformers to the existing modules or by creating new modules. To create a new module, simply create a new `.m` file in the `stdlib` directory and add your functions and transformers. Then, import your module in the `core.m` file to make it available to all programs that use the standard library.
