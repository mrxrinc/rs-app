// Arrays - Fixed list where elements are the same data types
//
// Arrays are stack allocated
//
// Arrays are useful when you want to have a collection of items of the same data type
//
// Arrays are not as flexible as vectors
//
// Vectors are more commonly used than arrays
//
// Vectors are similar to arrays, but they can grow or shrink in size
//
// Arrays are useful when you want your data allocated on the stack rather than the heap
//
// Arrays are useful when you want to ensure you always have a fixed number of elements
//
// Arrays are useful when you want to have a collection of items of the same data type

use std::mem;

pub fn run() {
    // Arrays
    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", numbers);

    // Re-assign value
    numbers[2] = 20;
    println!("{:?}", numbers);

    // Single value
    let single_value = [3; 5];
    println!("{:?}", single_value);

    // Get single val
    println!("Single Value: {}", single_value[0]);

    // Get array length
    println!("Array Length: {}", numbers.len());

    // Arrays are stack allocated ( each item occupies a fixed amount(= 4) of memory)
    println!("Array occupies {} bytes", mem::size_of_val(&numbers));

    // Get slice
    let slice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}", slice);
}