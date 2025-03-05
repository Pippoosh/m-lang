// math.m - Standard mathematical functions and transformers

// Basic arithmetic functions
fn abs(x) {
    if x < 0 {
        return -x
    }
    return x
}

fn max(a, b) {
    if a > b {
        return a
    }
    return b
}

fn min(a, b) {
    if a < b {
        return a
    }
    return b
}

fn pow(base, exponent) {
    if exponent == 0 {
        return 1
    }
    
    result = 1
    for i in range(0, exponent) {
        result = result * base
    }
    return result
}

fn factorial(n) {
    // Iterative implementation to avoid stack overflow
    if n <= 1 {
        return 1
    }
    
    result = 1
    for i in range(2, n + 1) {
        result = result * i
    }
    return result
}

fn is_even(n) {
    return (n % 2) == 0
}

fn is_odd(n) {
    return (n % 2) != 0
}

// Mathematical transformers
transformer abs() {
    if applied < 0 {
        return -applied
    }
    return applied
}

transformer square() {
    return applied * applied
}

transformer cube() {
    return applied * applied * applied
}

transformer sqrt() {
    // Simple approximation using Newton's method
    if applied <= 0 {
        return 0
    }
    
    x = applied
    y = 1
    
    // Just a few iterations for approximation
    for i in range(0, 10) {
        y = (y + x / y) / 2
    }
    
    return y
}

transformer negate() {
    return -applied
}

transformer increment() {
    return applied + 1
}

transformer decrement() {
    return applied - 1
}

// Print a message to show the math library was loaded
print("Math library loaded successfully")
