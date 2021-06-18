// write a function that takes a string and returns the first word it finds in that string. 
// If the function doesn’t find a space in the string, the whole string must be one word, 
// so the entire string should be returned.
fn main() {
    let mut s = String::from("hello world");
    let word = first_word(&s); // word will get the value 5
    let word_using_slice = first_word_using_slice(&s);

    println!("The value of word_using_slice is {}", word_using_slice);
    s.clear(); // this empties the String, making it equal to ""

    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 6 with, word is now totally invalid
    // The code compiles without any errors but imagine if we have starting and ending indecies both
    println!("The value of word is {}", word);
    // this will generate error: as s is cleared("mutable") while &s is immutable
    /* println!("The value of word_using_slice is {}", word_using_slice); */
    
    // ****String Slices as Parameters
    let my_string = String::from("hello world");

    // first_word_slices_parameters works on slices of `String`s
    let word = first_word_slices_parameters(&my_string[..]);
    println!("Using slices as paramters, word is {}", word);

    let my_string_literal = "hello world";

    // first_word_slices_parameters works on slices of string literals
    let word = first_word_slices_parameters(&my_string_literal[..]);
    println!("The value of word using slices of string literal is {}", word);

    // Because string literals *are* string slices already,
    // this works too, without the slices syntax!
    let word = first_word_slices_parameters(my_string_literal);
    println!("The value of word using string literal without slices is {}", word);
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

// Rewrite first_word fun to return a slice
fn first_word_using_slice(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]  // returns the whole string
}

//
fn first_word_slices_parameters(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
          return &s[0..i];
        }
    }
    &s[..] // returns the whole string
}
