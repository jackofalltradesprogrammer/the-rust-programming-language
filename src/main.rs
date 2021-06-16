// write a function that takes a string and returns the first word it finds in that string. 
// If the function doesn’t find a space in the string, the whole string must be one word, 
// so the entire string should be returned.
fn main() {
    let mut s = String::from("hello world");
    let word = first_word(&s); // word will get the value 5
    s.clear(); // this empties the String, making it equal to ""

    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 6 with, word is now totally invalid
    // The code compiles without any errors but imagine if we have starting and ending indecies both
    println!("The value of word is {}", word);
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}
