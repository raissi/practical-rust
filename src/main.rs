fn main() {

    //1. Functions:
    let age = 30;
    print_double(age); //Let's call a function that prints the double of age

    let double_age = compute_double(age); // A call to a function that returns the double of its param
    println!("The double of {} is: {}", age, double_age);

    let a = 75;
    let b = 110;

    let max = max(a, b);
    println!("The max between {} and {} is: {}", a, b, max);


    // 2. Loops:
    // Sum pair numbers between 1 and 20

    //Using a for loop
    let mut sum = 0;
    for i in 1..21 {  // Note: exclusive on right side
        if i % 2 == 0 {
            sum += i;
        }
    }
    println!("Sum of pair numbers by for loop is: {}", sum);

    //Using a while loop:
    let mut sum_by_while = 0;
    let mut counter = 1;
    while counter < 21 {

        if counter % 2 == 0 {
            sum_by_while += counter;
        }
        counter += 1;
    }

    println!("Sum of pair numbers by while loop is: {}", sum_by_while);

    //Using a loop:
    let mut sum_by_loop = 0;
    let mut counter = 1;

    loop {
        if counter % 2 == 0 {
            sum_by_loop += counter;
        }
        counter += 1;
        if counter > 20 {
            break;
        }
    }

    println!("Sum of pair numbers by loop is: {}", sum_by_loop);

    // 3. Arrays:
    
    let an_array = [1, 2, 3, 5, 7, 11];
    for prime in an_array { // For each element in the array
        println!("{}", prime);
    }
    // Print 3rd element:
    println!("The 3rd prime is: {}", an_array[2]); //Indices start with 0
    
    // Note that arrays are by default immutable (like everythins else in Rust :) 
    // Note also that arrays in Rust are of FIXED size
    // an_array[1] = 13; => would raise a compilation error. 
    let mut an_array = [1, 2, 3, 5, 7, 11];
    an_array[1] = 13; // Replace second element with 13
    println!("{:?}", an_array);

    // Get a slice of an array (we will be back to slices):
    let a_part = &an_array[1..3];
    println!("{:?}", a_part);

    
    // When illegal access if possible o detect, like here, the compilation will fail:
    // println!("{:?}", an_array[7]);

}

/// This is very simple function that only prints a message containing the double of its param
/// Note that Rust prefers rather snake_case than camelCase for naming variables, functions etc...
fn print_double(i: i32) { // 
    println!("The double of {} is {}", i, i*2)  // Note that when it's the last line, no need to put the ;
}

/// This is another very simple function that gives back the double of its param
/// Note that there is no need for 'return' and ';' when it's the last line 
fn compute_double(i: i32) -> i32 {
    i * 2
}


/// This function returns the max of two integers
fn max(first: i32, second: i32) -> i32 {
    if first >= second {
        return first;
    }
    second
}

