// string.m - String manipulation functions and transformers

// String functions
fn concat(a, b) {
    return a + b
}

fn repeat(str, times) {
    result = ""
    for i in range(0, times) {
        result = result + str
    }
    return result
}

// String transformers
transformer length() {
    count = 0
    for c in applied {
        count = count + 1
    }
    return count
}

transformer uppercase() {
    // This is a simplified implementation since we don't have character codes
    // In a real implementation, you would convert each character to uppercase
    return applied
}

transformer lowercase() {
    // This is a simplified implementation since we don't have character codes
    // In a real implementation, you would convert each character to lowercase
    return applied
}

transformer reverse() {
    // Reverse a string
    result = ""
    for c in applied {
        result = c + result
    }
    
    return result
}

transformer trim() {
    // This is a simplified implementation
    // In a real implementation, you would trim whitespace from start and end
    return applied
}

// Print a message to show the string library was loaded
print("String library loaded successfully")
