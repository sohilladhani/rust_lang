fn main() {
    /* Slice type */

    /* Slice type does not have ownership. Slices let you reference a
     * contiguous sequence of elements in a collection rather than the whole
     * collection.
     * */

    /* To understand its use, let us consider a programming problem.
     *
     * Write a function that takes a string and returns the first word
     * it finds in that string. If the function doesn't find a space in
     * the string, the whole string must be one word, so the entire string
     * should be returned. */

    let mut s = String::from("hello world");

    // prints the index of the end of the first word
    let index = first_word(&s);
    println!("index is {}", index);

    s.clear(); // clears the string to ""

    /* But when above code is executed, value of index is till 5, essentially
     * detaching it from the state of the string 's'.
     *
     * To tackle this problem of having to ensure sync between data and info
     * extracted on the basis of data, slice data type could be used. */

    /* String Slices */

    /* A string slice is a reference to part of a String, and it looks like
     * this: */

    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("{}, {}", hello, world);

    /* Rather than a reference to the entire string, it's a reference to a
     * portion of a string.
     *
     * We can create slices using a range within square brackets using form
     * [<starting_index>..<ending_index>]. ending_index is exclusive.
     *
     * Internally the slice data structure stores the start position and the
     * length of the slice. length = ending_index - starting_index.
     *
     * In the case of 'world' variable above, it contains at slice starting at
     * the 7th byte of 's' with a length of 5. The ending_index is 11, so the
     * last position in the slice is 10.
     * */

    /* Other ways to represent slices are below */

    let s = String::from("hello");
    let slice = &s[0..2];
    println!("{}", slice);
    // is essentially same as
    let slice = &s[..2];
    println!("{}", slice);

    /* Another example */
    let s = String::from("world");
    let len = s.len();

    let slice = &s[0..len];
    println!("{}", slice);
    // is essentially same as
    let slice = &s[..];
    println!("{}", slice);

    /* Since we have all the info about slice now, let's rewrite first_word function.
     * The new function is called first_word_slice.
     *
     * The type that signifies a 'string slice' is '&str' */

    let s = String::from("hello world, I'm your ruler!");
    println!("{}", first_word_slice(&s));

    /* Now the returned value is made up of a reference to the starting
     * point of the slice and the number of elements in the slice. */

    /* Now if we try to clear the string using slices, it throws error. */

    let mut s = String::from("hello world");

    let first_word = first_word_slice(&s); //immutable reference

    // compilation error due to mutable reference after an immutable reference
    /* s.clear(); */

    println!("{}", first_word);
    // no compilation error
    s.clear();

    /* String literals are 'slices' */

    /* The type of 's' here is '&str'. It's a slice pointing to that specific
     * point of the binary. This is also why string literals are immutable;
     * '&str' is an immutable reference. */
    let s = "hello world";
    println!("{}", s);

    /* String Slices as Parameters */

    let my_string = String::from("hello world");

    // give whole string as a slice
    let word = first_word_slice_as_parameter(&my_string[..]);
    println!("{}", word);

    let my_string_literal = "hello literal";

    // also works on slices of string literals
    let word = first_word_slice_as_parameter(&my_string_literal[..]);
    println!("{}", word);

    // since string literals are also slices already,
    // following works too, without the slice syntax
    let word = first_word_slice_as_parameter(my_string_literal);
    println!("{}", word);

    /* Other Slices*/

    /* There are also other type of slices. Consider below array. */
    let a = [1, 2, 3, 4, 5];

    // the below slice has the type '&[i32]'
    let slice = &a[1..3];

    for e in slice {
        println!("{}", e);
    }
}

fn first_word_slice_as_parameter(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn first_word_slice(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn first_word(s: &String) -> usize {
    // We need to get String element by element
    // This will convert the String into an array of bytes
    let bytes = s.as_bytes();

    // Iterating over array using iter() method
    // enumerate wraps the result of iter and returns each element as a part
    // of a tuple in the form (<element index>, <reference to the element>)
    // We can destructure the tuple like everwhere else in Rust
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            //returns index of the end of the first word
            return i;
        }
    }

    s.len()
}
