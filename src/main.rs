use std::fmt::Debug;

fn main() {
    let test_str = "String literal";
    let test_string = String::from("String"); //Our complex datatype
    let number = 2;
    let float:f32 = 2.01;
    let tuple: (i32, i64) = (200, 300);

    print_function(test_string); //Our complex data type going into print
    println!("{test_string}"); //No longer valid: a borrowing error. Does not implement the Copy trait.
    print_function(test_str);
    println!("{test_str}");
    print_function(number);
    println!("{number}");
    print_function(float);
    println!("{float}");
    print_function(tuple);
    println!("{tuple:?}");

    let test_string = String::from("String"); //Our complex datatype
    print_function(&test_string); //Our complex data type going into print
    println!("{test_string}"); //Still valid!
}

fn another_function() {
    println!("{test_str}"); //Is no longer valid
}

fn print_function<T: Debug>(data: T) {
    println!("{:?}", data);
}