//Compound Data Types
//arrays, tuples, slices, strings (slice of str), structs, enums, unions
//Arrays: Fixed size list of elements of the same data type


fn arrays() {
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];

    //println!("Number Array {}", numbers); //Display format
    //error[E0277]: `[i32; 5]` doesn't implement `std::fmt::Display`
    //[i32; 5]` cannot be formatted with the default formatter

    println!("Number Array {:?}", numbers); //Debuggable format

    let numbers_type_inference = [1, 2, 3, 4, 5]; //Type inference
    println!("Number Array {:?}", numbers_type_inference);

    //Accessing elements
    let numbers_with_default_value_and_length = [1; 5]; //Array of 5 elements with value 1
    println!("Number Array {:?}", numbers_with_default_value_and_length);


    //let mixed_data_types = [1, 2, true, 3, 4.0, 5, "bingo"]; 
    //Mixed data types
    //error[E0308]: mismatched types


    let fruits: [&str; 4] = ["apple", "banana", "orange", "mango"];
    println!("Fruits {:?}", fruits);
    println!("Fruits {}", fruits[0]);
    println!("Fruits {}", fruits[1]);
    println!("Fruits {}", fruits[2]);
    println!("Fruits {}", fruits[3]);
}

//Tuples
//Group together values of different types
//Fixed length: Once declared, cannot grow or shrink in size
fn tuples() {
    let person: (bool, &str, &str, i8, f32) = (true, "John", "Doe", 30, 5.11);
    let person_type_inference = (true, "John", "Doe", 30, 5.11);
    println!("Person {:?}", person);
    println!("Person {} {} {} {} {}", person.0, person.1, person.2, person.3, person.4);
    println!("Person {:?}", person_type_inference);
    println!("Person {} {} {} {} {}", person_type_inference.0, person_type_inference.1, person_type_inference.2, person_type_inference.3, person_type_inference.4);

    //Array inside a tuple
    let array_inside_a_tuple = (true, "John", [1;5], 30, [3, 25, 42, 66, 211], 5.11);
    println!("Person with Array {:?}", array_inside_a_tuple);
}

//Slices, String slices, String
//Reference to a contiguous sequence of elements in a collection
//Slices are a reference to a contiguous sequence of elements in a collection
//For elements in a slice, memory is allocated adjacent to each other
//Slices do not have ownership, they are borrowed, therefore they cannot be modified
//Slices are used to pass a reference
fn slice(){

    let number_slice: &[i32] = &[1, 2, 3, 4, 5];
    println!("Number Slice {:?}", number_slice);

    let numbers: [i32; 5] = [1, 2, 3, 4, 5]; //Array
    let slice = &numbers[1..3]; //Slice of numbers from index 1 to 3
    println!("Slice of numbers array {:?}", slice);
    

    //str is an unsized type and therefore cannot be used directly
    //str represents a immutable sequence of UTF-8 bytes
    //str is stored as immutable data in the binary
    //&str is a pointer to a sequence of UTF-8 bytes (str)
    //str is a borrowed type and does not have ownership, tehrefore it cannot be modified
    //str is stored on the stack, not the heap. Therefore, it is faster to access than String but cannot be modified
    let bird_str_slice_pointers_slice: &[&str] = &["Eagle", "Sparrow", "Indian Peafowl", "Indian Pheasant", "Kingfisher", "Chicken"];
    println!("Birds Slices Slice {:?}", bird_str_slice_pointers_slice);
    
    //String is a growable, mutable, owned, UTF-8 encoded string type
    //String is an object that is stored on the heap
    //String is an owned type and has ownership, therefore it can be modified
    // String::from() is used to create a new String object from a string literal
    // to_string() is used to create a new String object from a string literal
    // to_string() is a method defined using the trait ToString
    let bird_string_objects_slice: &[String] = &["Eagle".to_string(), "Sparrow".to_string(), String::from("Indian Peafowl"),
     String::from("Indian Pheasant"), "Kingfisher".to_string(), "Chicken".to_string()];
    println!("Birds Strings Slice {:?}", bird_string_objects_slice);

    //Prefixing & to a String object creates a reference to the String object
    //The reference/pointer is stored in the slice below
    let bird_string_object_pointers_slice: &[&String] = &[&"Eagle".to_string(), &"Sparrow".to_string(), &"Indian Peafowl".to_string(),
     &"Indian Pheasant".to_string(), &"Kingfisher".to_string(), &"Chicken".to_string()];
    println!("Birds String Slices Slice {:?}", bird_string_object_pointers_slice);
}



fn main() {
    arrays();
    tuples();
    slice();
}