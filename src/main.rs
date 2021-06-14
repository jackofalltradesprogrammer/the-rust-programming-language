fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    // change(&s1);
    // `some_string` is a `&` reference, so the data it refers to cannot be borrowed as mutable.
    
    let mut s2 = String::from("hello");
    change(&mut s2);

    let len2 = calculate_length(&s2);

    println!("The length of '{}' is {}.", s2, len2);
    
//    Error: cannot borrow `s1` as mutable more than once at a time 
//    let r1 = &mut s1;
//    let r2 = &mut s1;
//    println!("{}, {}", r1, r2);

// We can use curyly brackets (new scope) to allow for multiple mutable references
    let mut s = String::from("hello");
    {
        let r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems
    let r2 = &mut s;
    println!("The value of r2 is {}, ", r2);
}

fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, nothing happens.

// References are not mutable for regular varaibles* & generates an error
// fn change(some_string: &String) {
//     some_string.push_str(", world");
// }

// Make it mutable to modify the reference
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
