// a slice is a reference to a continuous sequence of elements in a collection, a subset of the whole collection. since a slice is a kind
// of reference, it doesn't have ownership.

fn main() {
    let mut s = String::from("hello world");

    let word = first_word_basic(&s);

    let slice = first_word_slice(&s);

    println!("{word}");

    println!("{s}");
}

// a function to return the index of the first space in a string
fn first_word_basic(s: &String) -> usize {
    let bytes = s.as_bytes(); // converting string to array of bytes for element by element checking

    for (i, &item) in bytes.iter().enumerate() { // we're taking the index i and a reference to the element in that index
        if item == b' ' {
            return i; // if there is a space, we return early from the function and give the index of the space
        }
    }

    s.len() // if there is no space in the string, we reach this point and we return the length of the string, as there is no space
}


fn first_word_slice(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}