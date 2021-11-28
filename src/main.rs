fn main() {

    //Two types of Strings

    //1. String slices:
    // A string slice has a fixed size, and cannot be mutated. It is a reference to a sequence of UTF-8 bytes:
    //The type is: &'static str
    //the 'static is to say this value is saved inside our compiled program, and exists for the entire duration it runs
    //and we usually 
    let some_str = "this is a static string, known at compilation time";

    println!("{}", some_str);

    //We normally cannot access a str directly, but only through a &str reference.
    //This is because str is an unsized type (not dscussed here)

    //2. If we want mutable strings, we need to use String type:
    let mut s = String::from(some_str);
    s.push_str(". But a value can be appended to it using String");
    println!("{}", s);

    // We could transform &str to String:
    let mut s = some_str.to_string();
    s.push_str(". Another appended valueto original str");
    println!("{}", s);
    

    //String can be coerced to &str:
    let name = String::from("Foo");
    takes_slice(&name);

    //But the inverse can not be:
    //takes_string("ISSAT-K");  => This will not compile

    takes_string(&name);

    //Concat between a String + &str AND between String + &String:
    let s_1 = "Hello ";
    let s_2 = "World";

    let hello = String::from(s_1) + &String::from(s_2);
    println!("{}", hello);

    //String type exists "to support a mutable, growable piece of text, 
    //by allocating an amount of memory on the heap, unknown at compile time, to hold the contents" (from The Rust Programming Language)


    //Ownership
    //let hello_2 = hello;
    //println!("{}", hello);   => This would raise compilation error => This is why Rust exists

    let a = 35;
    let b = a;

    println!("A is: {} and b is: {}", a, b);
    //Compiles because a and b are integers. So when b = a, the integer in a (35) is simply copied into b
    // Integers are stack stored types
    //But for Strings, Rust does not copy the values (that may be very expensive), it only _copies_ the references
    //so the reference hello is copied into hello_2, and hello is also invalidated

    //Ownership and Functions
    let name = String::from("Foo");
    say_hello(name);

    //println!("the nae was {}", name);  
    //=> this would not compile since say_hello took ownership of name
    //if name was integer => Copy


    let first = String::from("Foo");
    let last = String::from("Bar");
    let name = compute_name(first, last);
    println!("Hello: {}", name);

    //println!("First name was: {}", first);
    //=> of course would not compile

    //But what if we still needed our first and last variables ?


    //Borrowing:

    let first = String::from("Foo");
    let last = String::from("Bar");
    let name = compute_name_2(&first, &last);

    println!("Hello: {}", name);
    println!("First name was: {} and last was: {}", first, last);
    // "These ampersands are references, and they allow you to refer to some value without taking ownership of it" (from the Rust book)

    //Mutable references:
    //As is everything in Rust by default, references are immutable.
    //so compute_name_2 can not change value of its params
    //If we need to do that, we need to declare it:

    let mut prefix = String::from("Mr.");
    let name = "Foo";
    append(&mut prefix, &name);

    println!("Hello: {}", prefix);
    //Avoid this, muting inside a function makes people need to read the function to know what it does

    //Only one mut reference at a time:
    let mut prefix = String::from("Mr.");
    
    let s_1 = &mut prefix;
    append(s_1, "Foo");
    let s_2 = &mut prefix; //from here s_1 is no more valid

    println!("s_1: {}", s_2);  //printing here s_1 will raise problem

    append(&mut prefix, "Bar"); //Another inline borrowing of prefix
    
    println!("Hello after multi borrowings: {}", prefix);


    //See dangle fn
    let d = dangle_demo();
    println!("from dangle: {}", d);
}


fn takes_slice(slice: &str) {
    println!("Hello (&str) {}", slice);
}

fn takes_string(s: &String) {
    println!("Hello (String): {}", s);
}

fn say_hello(name: String) {
    println!("Hello {}", name)
}

fn compute_name(first_name: String, last_name: String) -> String {
    first_name + " " + &last_name
    //values are dropped after fn returns here
}


/// Fn takes references to Strings instead of Strings themselves
fn compute_name_2(first_name: &String, last_name: &String) -> String {
    String::from(first_name) + " " + &last_name
    // first_name and last_name are just references, they do not have ownership, so their values are not dropped after return
}


fn append(prefix: &mut String, name: &str) {
    prefix.push_str(" ");
    prefix.push_str(name);
}

fn dangle_demo() -> String {
    let s = String::from("Hello");
    s  //Here ownership is moved out to the caller
    //If we were to return &s (and &String as return type of the function)
    //we couldn't, since ownership would be dropped with return
}