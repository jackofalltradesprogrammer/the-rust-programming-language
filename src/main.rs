fn main() {
    let s1 = String::from("hello"); // :: is used to namespace that particular function
    let s2 = s1;

    s.push_str(", world!"); 
    // push_str() appends a literal to a String

    // compile error as s1 is no longer valid
    println!("{}, world!", s1); 
}
