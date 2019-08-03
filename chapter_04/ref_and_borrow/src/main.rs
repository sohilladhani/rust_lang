fn main() {
    /* Instead of taking ownership of the value, one can pass reference of
     * the object to a function to use the value. This removes the problem
     * of providing a variable to a function and returning the value of that
     * variable back to the calling code. */

    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("Length of {} is {}.", s1, len);

    /* The ampersands in the above code are 'references', which allow you to
     * refer to some value without taking ownership of the value.
     *
     *
     * Below is the new memory diagram:
     *
     *        s                          s1                         heap
     * ----------------   ----------------------   -----------------
     * | name | value |   | name       | value |   | index | value |
     * ----------------   ----------------------   -----------------
     * | ptr  | --------->| ptr        | --------->|   0   |   h   |
     * ----------------   ----------------------   -----------------
     *                    | len        |   5   |   |   1   |   e   |
     *                    ----------------------   -----------------
     *                    | capacity   |   5   |   |   2   |   l   |
     *                    ----------------------   -----------------
     *                                             |   3   |   l   |
     *                                             -----------------
     *                                             |   4   |   o   |
     *                                             -----------------
     *
     *
     * This way of having references as function parameters is called 'borrowing',
     * since the function is just borrowing value rather than taking ownership
     * of it.
     *
     * One cannot (or should not) modify what is borrowed in real life. Same is
     * true in Rust. The following code produces compilation errors since the
     * function is trying to edit the borrowed value.
     * */

    /* let s = String::from("hello"); */
    /* change (&s); */
 // Compilation error

    /* Mutable references */

    let mut s = String::from("hello");
    change_mut_string(&mut s);
    println!("New value of s is {}", s);

    /* Limitation: One cannot have more than 1 mutable references to a value */
    let mut a = String::from("a");
    let r1 = &mut a;
    /* let r2 = &mut a; */

    /* println!("r1 is {}, r2 is {}", r1, r2);*/
 // Compilation error stating
 // second mutable borrow occurs here

    println!("r1 is {}", r1);
    /* The benefit of this restriction is that Rust can prevent data races at
     * compile time.
     *
     * A data race is a race condition which happens when these 3 behaviors
     * occur:
     *
     * 1. 2 or more pointers access the same data at the same time.
     * 2. At least 1 of the pointers is being used to write the data.
     * 3. There's not mechanism being used to synchronize access to the data*/

    /* To solve the above problem, we can create a new scope allowing multiple
     * mutable references, just not simulataneous ones:
     * */

    {
        // new scope for r2, a mutable reference
        let r2 = &mut a;
        println!("r2 is {}", r2);
    }

    /* Combining mutable and immutable references */

    /* There are some rules for combining mutable and immutable references.
     * Consider below code.
     * */

    let mut st = String::from("hello");

    let r1 = &st; // no problem
    let r2 = &st; // no problem
                  /* let r3 = &mut st; */  // BIG problem

    /* println!("{}, {}, {}", r1, r2, r3); */
 // compilation error stating
 // that 2 borrows occur, 1 is
 // mutable and other is immutable

    println!("{}, {}", r1, r2); // compiles with no problem with a warning

    /* We cannot have a mutable reference while we have an immutable one.
     * Users of immutable references do not expect the value to change
     * suddenly. However mutiple immutable references are okay since we
     * are just reading data from them and not modifying the data.
     * */

    /* How to use both a mutable and an immutable reference then?
     *
     * Note: The scope of a reference starts from where it is introduced and
     * continues through the last time that reference is used.
     *
     * For example: the code below will compile because the last usage of
     * immutable references occurs before the mutable reference is introduced.
     * */

    let mut name = String::from("sohil");
    let r1 = &name;
    let r2 = &name;

    println!("{}, {}", r1, r2);
    // r1 and r2 are no longer used after this point

    let r3 = &mut name;
    r3.push_str(" ladhani");
    println!("{}", r3);

    /* In short, if a value is mutable, the immutable references to that
     * value should be used before creating mutable references. One can
     * have as many immutable references as one can have before a mutable
     * reference, since immutable references are read-only representation
     * of a value.
     *
     * Moreover, one cannot create multiple mutable references before using
     * those mutable references. One has to use (or modify and use) the value
     * using mutable reference before creating another one to the same valueas one can have before
     * a mutable
     * reference, since immutable references are read-only representation
     * of a value.
     *
     * Moreover, one cannot create multiple mutable references before using
     * those mutable references. One has to use (or modify and use) the value
     * using mutable reference before creating another one to the same value,
     * since mutable references can modify the value.
     * */

    /* Dangling References */
    /* Rust compiler ensures that there are no dangling references by making
     * sure that the reference would go out of scope before the data it points
     * to. */

    let ref_to_nothing = dangle();

    println!("{}", ref_to_nothing);

    /*
     * Rule of References:
     *
     * 1. At any given time, you can have either 1 mutable reference or any
     *    number of immutable references.
     * 2. References must always be valid.
     *
     * */
}

fn calculate_length(s: &String) -> usize {
    // s is a reference to a String
    s.len()
} // Here 's' goes out of scope. But because it does not own what it refers to,
  // nothing happens

/* fn change(some_string: &String) {
    some_string.push_str(", world!"); // compilation error stating: some_string
                                      // is a `&` reference, so the data it
                                      // refers to cannot be borrowed as mutable
} */

fn change_mut_string(some_string: &mut String) {
    some_string.push_str(", world!");
}

/* fn dangle() -> &String {    // dangle returns a reference to a String

    let s = String::from("hello");  // s is a new string

    &s;     // we return a reference to the String, s
} */
 // Here, 's' goes out of scope and so does the reference. Its memory goes
 // away. Rather use the below code and return just the string not
 // the reference

fn dangle() -> String {
    let s = String::from("hello");
    s
}
