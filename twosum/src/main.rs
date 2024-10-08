// Function declaration
fn two_sum(nums: &[i32], target: i32) -> Option<(usize, usize)> {
    /*
    Function declaration breakdown:
    - `fn`: Keyword to declare a function
    - `two_sum`: Function name
    - `(nums: &[i32], target: i32)`: Parameters
      - `nums: &[i32]`: 
        - `nums` is the parameter name
        - `&[i32]` is its type: a reference to a slice of i32
      - `target: i32`:
        - `target` is the parameter name
        - `i32` is its type: a 32-bit signed integer
    - `-> Option<(usize, usize)>`: Return type
      - `Option`: An enum that represents optional values
      - `<(usize, usize)>`: The type parameter for Option
        - `usize`: An unsigned integer type with the same number of bits as the platform's pointer type
                   (e.g., 64 bits on a 64-bit system). Commonly used for indexing and size operations.
        - The parentheses create a tuple of two `usize` values

    Notes on Option<(usize, usize)>:
    - The function returns an Option that, when it's Some, contains a tuple of two usize values
    - Option can be set up with any return type, be it primitive or complex (e.g., arrays, vectors, structs)
    - Examples of Option usage:
      - Option<i32>: An optional integer
      - Option<String>: An optional string
      - Option<Vec<f64>>: An optional vector of floating-point numbers

    Notes on usize for array indices:
    - usize is guaranteed to be large enough to hold any memory address or size on the target platform
    - On 32-bit systems, usize is 32 bits (same as u32)
    - On 64-bit systems, usize is 64 bits (same as u64)
    - Using usize ensures that the code can index any array, regardless of the platform
    - While u32 could work for many cases, usize is the idiomatic choice in Rust for array indices and sizes

    Memory layout visualization:

    Stack:
    [
        nums: &[i32] (fat pointer),
        |   ptr     | len     |  (16 bytes on 64-bit systems)
        +----------+--------+
        | 0x1234.. | 4      |  (example values)
        +----------+--------+
        target: i32,
        |   value   |  (4 bytes)
        +-----------+
        | 9         |  (example value)
        +-----------+
    ]

    Heap:
    [
        0x1234..: [2, 7, 11, 15]  (actual array data pointed to by nums)
                  ^  ^  ^   ^
                  |  |  |   |
                  0  1  2   3     (indices, type: usize)
    ]

    Fat Pointer vs Regular Pointer:
    - Regular pointer: Just contains the memory address (8 bytes on 64-bit systems)
      [  address  ]
      0x1234567890
    
    - Fat pointer: Contains both the memory address and additional metadata
      For &[i32]:
      [  address  |  length  ]
      0x1234567890    4
      
      1. Pointer to the data (8 bytes): Memory address of the first element
      2. Length of the slice (8 bytes): Number of elements in the slice
    
    Slice vs Reference:
    - Reference (&T): 
      - Points to a single value of type T
      - Size: Same as a regular pointer (8 bytes on 64-bit systems)
      - Example: &i32 points to a single i32 value
    
    - Slice (&[T]): 
      - Points to a contiguous sequence of T
      - Size: Same as a fat pointer (16 bytes on 64-bit systems)
      - Includes length information
      - Can refer to a portion of an array or vector
      - Example: &[i32] can point to multiple i32 values
    */

    // The `for` loop uses the range syntax `0..nums.len()`
    // `..` creates a range that is inclusive on the left and exclusive on the right
    // So `0..nums.len()` means all indices from 0 up to, but not including, nums.len()
    for i in 0..nums.len() {
        // `nums.len()` accesses the length stored in the fat pointer
        // This length is stored alongside the pointer in the slice's metadata

        // `j` starts from `i + 1` to avoid duplicate pairs and comparing an element with itself
        for j in (i + 1)..nums.len() {
            // Rust uses short-circuit evaluation for boolean operations
            // If `nums[i] + nums[j] == target` is true, `Some((i, j))` is immediately returned
            // The semicolon is omitted here because this is an expression, not a statement
            // In Rust, the last expression in a block is implicitly returned
            if nums[i] + nums[j] == target {
                // `Some` is a variant of the `Option` enum
                // `Some((i, j))` constructs a `Some` value containing a tuple of two `usize` values
                // This represents a successful result with the indices of the two numbers
                return Some((i, j))
            }
        }
    }

    // `None` is also a variant of the `Option` enum
    // It represents the absence of a value (i.e., no solution found)
    // `None` is also an expression, so no semicolon is needed
    // This is equivalent to `return None;`, but the `return` keyword is typically omitted for the last expression
    None
}

fn test_two_sum() {
    /*
    Array memory layout:

    Stack:
    [
        nums: [i32; 4],  (16 bytes for the array elements)
        +-----------+-----------+-----------+-----------+
        |     2     |     7     |    11     |    15     |
        +-----------+-----------+-----------+-----------+
        ^           ^           ^           ^
        |           |           |           |
        0           1           2           3    (indices, type: usize)
    ]

    Array vs Vector:
    - Array ([i32; 4]) advantages over vector (Vec<i32>) in this case:
      1. Memory efficiency: Stored entirely on the stack, avoiding heap allocation.
      2. Performance: Stack allocation is generally faster than heap allocation.
      3. Size known at compile-time: Exactly four elements needed.
      4. Immutability: Arrays are immutable by default, matching input requirements.
      5. Zero-cost abstraction: No runtime overhead compared to raw elements.

    Array limitations compared to vectors:
    - Fixed size: Cannot grow or shrink at runtime.
    - Size must be known at compile-time.
    - Less convenient for dynamic data.

    An array is more suitable here due to the fixed, known size of the input data.
    */
    let nums: [i32; 4] = [2, 7, 11, 15];
    
    let target = 9;

    // When calling `two_sum(&nums, target)`:
    // - `&nums` creates a slice (fat pointer) to the array data
    // - `target` is copied (i32 implements Copy trait)
    let result = two_sum(&nums, target);

    /*
    Passing a reference (&nums) instead of the whole array:
    1. Efficiency: Avoids copying the entire array (though small arrays might be optimized by the compiler)
    2. Flexibility: The function can accept slices from various sources (arrays, vectors, etc.)
    3. Borrowing semantics: The caller retains ownership of the data
    4. Zero-cost abstraction: Using a slice has no runtime cost compared to passing the raw parts
    */

    /*
    Notes on `if let Some((i, j)) = result`:
    - `result` is an Option<(usize, usize)>
    - This line performs pattern matching on the Option
    - If `result` is `Some`, it extracts the tuple (i, j) from it
    - If `result` is `None`, this branch is not executed

    Equivalent match expression:
    match result {
        Some((i, j)) => {
            println!("Solution found: indices {} and {}", i, j);
        },
        None => {
            println!("No solution found");
        }
    }

    The indices i and j are relative to the original array, allowing access to the actual values:
    if let Some((i, j)) = result {
        println!("Solution found: indices {} and {}", i, j);
        println!("Values: {} and {}", nums[i], nums[j]);
    } else {
        println!("No solution found");
    }
    */

    // Pattern matching with `if let`
    if let Some((i, j)) = result {
        println!("Solution found: indices {} and {}", i, j);
    } else {
        println!("No solution found");
    }
}

fn main() {
    // `main` function: entry point of the program
    // At program start, the runtime sets up the stack and heap:
    // 
    // Stack: Fixed-size memory for local variables, function calls, and primitive values
    // [main's stack frame]
    //
    // Heap: Dynamic memory for data with unknown size at compile time or size that might change
    // [empty initially]
    
    test_two_sum();
    
    // When `main` returns:
    // - All stack-allocated variables are dropped
    // - Using an array instead of a Vec, all memory is automatically freed when it goes out of scope
    // - No heap allocation or deallocation is involved in this case
}

/*
Additional Rust concepts:

1. Ownership and Borrowing:
   - `&nums` in `two_sum(&nums, target)` creates a shared borrow
   - The borrow checker ensures this reference doesn't outlive `nums`
   - Multiple shared borrows (&T) can coexist, but not with a mutable borrow (&mut T)
   - This system prevents data races and ensures memory safety at compile time

2. Slices vs Arrays vs Vectors:
   - Array ([T; N]): 
     - Fixed-size, stored entirely on stack
     - Example: let arr: [i32; 4] = [1, 2, 3, 4];
     - Size is part of its type and must be known at compile time
   - Vector (Vec<T>): 
     - Dynamically-sized, stores data on heap
     - Can grow or shrink at runtime
     - Provides methods like push(), pop(), etc.
   - Slice (&[T]): 
     - View into either an array or vector, doesn't own data
     - Created from arrays or vectors: &arr[..] or &vec[..]
     - Allows working with a portion of a collection efficiently

3. Option<T> enum:
   enum Option<T> {
       Some(T),
       None,
   }
   - Used for values that may or may not exist
   - Prevents null pointer exceptions common in other languages
   - `Some(T)` variant contains a value of type T
   - `None` variant represents the absence of a value
   - Forces explicit handling of the possibility of absence

   Null Pointer Exceptions and Option:
   - Many languages use null to represent absent values, leading to null pointer exceptions
   - Null pointer exceptions are problematic because:
     1. They can crash programs
     2. They're often runtime errors, not caught at compile time
     3. They can lead to security vulnerabilities
   - Rust's Option type forces explicit handling of potentially absent values
   - This moves many errors from runtime to compile-time, increasing program safety and reliability
   - Example in another language:
     String s = null;
     int length = s.length();  // Crashes with NullPointerException
   - Equivalent Rust code wouldn't compile, forcing handling of the None case

4. Semicolons and Expressions:
   - Expressions (like `Some((i, j))` and `None`) don't need semicolons
     - They evaluate to a value and can be used as part of larger expressions
   - Statements (like `let x = 5;`) do need semicolons
     - They perform an action but don't evaluate to a value
   - The last expression in a block is its return value if the semicolon is omitted

5. Memory Management:
   - Rust uses RAII (Resource Acquisition Is Initialization)
   - When a value goes out of scope, its destructor is called and its memory is freed
   - This applies to both stack and heap memory
   - For types that own heap memory (like Vec<T>):
     1. The stack-allocated part is dropped
     2. The destructor frees the heap-allocated buffer
   - No garbage collector or manual memory management needed

6. Compile-time Guarantees:
   - The borrow checker ensures memory safety and prevents data races at compile time
   - This eliminates entire classes of bugs common in other systems programming languages
   - Examples of prevented issues:
     - Use after free
     - Double free
     - Null pointer dereferences
     - Buffer overflows
     - Iterator invalidation

7. usize type:
   - Unsigned integer type with size matching the platform's pointer type
   - On 32-bit platforms: 32 bits
   - On 64-bit platforms: 64 bits
   - Used for:
     - Array indexing
     - Representing sizes of in-memory data structures
     - Pointer arithmetic
   - Guarantees that it can hold the size of any object in memory
   - While u32 might work for many cases, usize ensures compatibility across different platforms
   - Using usize for indices allows code to work with arrays of any size the platform can support

8. Stack vs Heap:
   - Stack:
     - Fast allocation and deallocation (just moving a stack pointer)
     - Limited in size (often a few MB)
     - Automatic memory management (values freed when they go out of scope)
     - Used for:
       - Local variables
       - Function call data (return addresses, parameters, local variables)
       - Small, fixed-size values
   - Heap:
     - Slower allocation and deallocation (requires memory allocation algorithms)
     - Limited only by available system memory
     - Manual memory management in many languages (but handled by Rust's ownership system)
     - Used for:
       - Dynamically sized data
       - Long-lived data that outlives the function that created it
       - Large amounts of data

9. Rust's Slice Type:
   - Slices are a view into a contiguous sequence of elements in an array or vector
   - They're a dynamically-sized view, represented by a fat pointer:
     - A pointer to the data
     - The length of the slice
   - Slices allow safe and efficient work with a portion of an array or vector
   - They can be mutable (&mut [T]) or immutable (&[T])
   - Slices are fundamental to Rust's borrowing system, allowing safe, efficient references to array data
*/
