// array.m - Array manipulation functions and transformers

// Array functions
fn create_array(size, default_value) {
    arr = []
    for i in range(0, size) {
        arr = arr + [default_value]
    }
    return arr
}

fn array_get(arr, index) {
    return arr[index]
}

fn array_set(arr, index, value) {
    // Create a new array with the value at the specified index
    result = []
    for i in range(0, arr.length()) {
        if i == index {
            result = result + [value]
        } else {
            result = result + [arr[i]]
        }
    }
    return result
}

// Array transformers
transformer length() {
    count = 0
    for item in applied {
        count = count + 1
    }
    return count
}

transformer sum() {
    total = 0
    for item in applied {
        total = total + item
    }
    return total
}

transformer average() {
    total = 0
    count = 0
    
    for item in applied {
        total = total + item
        count = count + 1
    }
    
    if count == 0 {
        return 0
    }
    
    return total / count
}

transformer map(func) {
    result = []
    for item in applied {
        result = result + [func(item)]
    }
    return result
}

transformer filter(predicate) {
    result = []
    for item in applied {
        if predicate(item) {
            result = result + [item]
        }
    }
    return result
}

transformer reverse() {
    result = []
    for item in applied {
        result = [item] + result
    }
    return result
}

transformer sort() {
    // This is a simplified implementation that creates a new sorted array
    // We'll use a simple selection sort algorithm
    arr = applied
    n = arr.length()
    result = []
    
    // Create a copy of the array
    for item in arr {
        result = result + [item]
    }
    
    // Simple selection sort
    for i in range(0, n) {
        min_idx = i
        min_val = result[i]
        
        // Find the minimum element
        for j in range(i + 1, n) {
            if result[j] < min_val {
                min_idx = j
                min_val = result[j]
            }
        }
        
        // Swap the found minimum element with the element at index i
        if min_idx != i {
            // We can't directly modify the array, so we'll create a new one
            temp = result[i]
            result = array_set(result, i, result[min_idx])
            result = array_set(result, min_idx, temp)
        }
    }
    
    return result
}

// Print a message to show the array library was loaded
print("Array library loaded successfully")
