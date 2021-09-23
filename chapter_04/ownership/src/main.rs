/*
 * Ownership rules:
 * 1. Each value in Rust has a variable that’s called its owner.
 * 2. There can only be 1 owner at a time.
 * 3. And, when the owner goes out of scope, the value is dropped
 */

fn main() {
    println!("I rule the world!");

    /* Scope: The range within the program for which an item is valid */
    {
        // 's' is not valid here since it is not
        // declared yet

        let s = "hey"; // 's' is valid from this point forward
        println!("{}", s);
    } // 's' becomes invalid since the scope is over

    /* println!("{}", s); */
 // compilation error

    {
        /* String 'type' creates a text on the heap, which is unknown at the
         * compile time. 'from' method helps us to create the text from the
         * string literal. This type of string can be mutated.
         */
        let mut s = String::from("hello");
        s.push_str(", world");
        println!("{}", s);
    }
    /* Rust calls 'drop' function implicitly to free memory when the scope
     * ends for a variable. In Rust, the memory is automatically returned
     * to the OS once the variable that owns it goes out of scope. */

    /* Ways Variables and Data interact: Move */

    let x = 5;
    let y = x;
    println!("{}", y);

    /* The above code compiles as we are binding '5' to 'x' and making a copy
     * of the value in 'x' and binding it to 'y' and both equal to '5'. This
     * happens because integers are simple values with a known, fixed size,
     * and these two '5' are pushed to stack. */

    let s1 = String::from("hello");
    let s2 = s1;
    println!("{}", s2);

    /* The above code, "let s2 = s1", does not make a copy of the value in s1,
     * and bind it to s2. A String type is made of 3 parts.
     * (1) A pointer to the memory that holds the content of the String.
     * (2) Length of the String: bytes used in memory
     * (3) Capacity of the String: total bytes received from OS
     * This group of data is stored on the stack. */

    /*        s1                           heap
     * ----------------------       -----------------
     * | name       | value |       | index | value |
     * ----------------------       -----------------
     * | ptr        | ------------->|   0   |   h   |
     * ----------------------       -----------------
     * | len        |   5   |       |   1   |   e   |
     * ----------------------       -----------------
     * | capacity   |   5   |       |   2   |   l   |
     * ----------------------       -----------------
     *                              |   3   |   l   |
     *                              -----------------
     *                              |   4   |   o   |
     *                              -----------------
     *
     *
     *
     * let s2 = s1;
     *
     * This code copies the content of the String, i.e. the pointer, the length
     * and capacity on the stack. The content of the heap memory is not copied.
     * The new memory content looks like below.
     *
     *          s1                         heap
     * ----------------------       -----------------
     * | name       | value |       | index | value |
     * ----------------------       -----------------
     * | ptr        | ------------->|   0   |   h   |
     * ----------------------   |   -----------------
     * | len        |   5   |   |   |   1   |   e   |
     * ----------------------   |   -----------------
     * | capacity   |   5   |   |   |   2   |   l   |
     * ----------------------   |   -----------------
     *                          |   |   3   |   l   |
     *                          |   -----------------
     *                          |   |   4   |   o   |
     *                          |   -----------------
     *          s2              |
     * ----------------------   |
     * | name       | value |   |
     * ----------------------   |
     * | ptr        | -----------
     * ----------------------
     * | len        |   5   |
     * ----------------------
     * | capacity   |   5   |
     * ----------------------
     *
     *
     * But this is a problem. Because, when s1 and s2 go out of scope, they
     * will both try to free the same memory. This is known as 'double free'
     * error. This leads to memory corruption leading to security vulnerabilites.
     *
     * What Rust should do now to ensure memory safety?
     *
     * What Rust does is, as soon as s2 is assigned s1 (let s2 = s1), Rust
     * considers s1 to be 'invalid'. Hence, there is no question of invalidating
     * s1 when it goes out of scope.
     *
     * To see it in action, see below code.
     * */

    /* println!("{}", s1); */
 // compilation error stating "value borrowed here after
 // move"

    /* It is known 'shallow copy', since we are just copying the info about
     * the data rather than the date itself. Since Rust invalidates the first
     * variable, it is called 'move'. Here, we could say that s1 was 'moved'
     * into s2.
     *
     * The updated figure looks like below:
     *
     *    s1(invalidated)                  heap
     * ----------------------       -----------------
     * |/name///////|/value/|       | index | value |
     * ----------------------       -----------------
     * |/ptr////////|xxxxxxxx   --->|   0   |   h   |
     * ----------------------   |   -----------------
     * |/len////////|///5///|   |   |   1   |   e   |
     * ----------------------   |   -----------------
     * |/capacity///|///5///|   |   |   2   |   l   |
     * ----------------------   |   -----------------
     *                          |   |   3   |   l   |
     *                          |   -----------------
     *                          |   |   4   |   o   |
     *                          |   -----------------
     *          s2              |
     * ----------------------   |
     * | name       | value |   |
     * ----------------------   |
     * | ptr        | -----------
     * ----------------------
     * | len        |   5   |
     * ----------------------
     * | capacity   |   5   |
     * ----------------------
     *
     *
     * It's a design choice of Rust which implies that Rust will never automatically
     * create a 'deep copy', resulting in an inexpensive runtime performance.
     *
     * */

    /* Ways Variables and Data Interact: Clone */

    /* If one really wants to deeply copy the heap data of the String, 'clone'
     * method could be used. So the code now becomes: */

    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);

    /* This way, one can know by looking at the code that expensive operations
     * like 'clone' is used. */

    /* Stack-Only Data: Copy*/

    /* When using integers, 'clone' method is not needed since the value is
     * known at the compile time and are stored entirely on stack. So copying
     * data is pretty quick.
     *
     * */
    let x = 5;
    let y = x; // no 'clone' is needed
    println!("x is {}, y is {}", x, y);

    /* Rust has a special annotation called the Copy trait that we can place
     * on types like integers that are stored on the stack. If a type has the
     * Copy trait, an older variable is still usable after assignment. Rust
     * won’t let us annotate a type with the Copy trait if the type, or any
     * of its parts, has implemented the Drop trait. If the type needs something
     * special to happen when the value goes out of scope and we add the Copy
     * annotation to that type, we’ll get a compile-time error.
     *
     * Check docs to know about the type that are 'Copy'.
     * Some eg:
     * 1. All integers like u32
     * 2. Boolean types
     * 3. Floating point types
     * 4. Character types
     * 5. Tuples (only if contain types are Copy) e.g (i32, i32) is Copy.
     *    (i32, String) is not.
     * */

    /* Ownership and Functions */

    /* The semantics for passing a value to a function are similar to those
     * for assigning a value to a variable. Passing a variable to a function
     * will move or copy, like what an assignment does.
     *
     * Take below code for example.
     * */

    let s = String::from("Sohil"); // 's' comes in scope
    takes_ownership(s); // 's' moved into function
                        // 's' become invalid here
                        /* println!("{}", s); */        // compilation error

    let x = 5; // 'x' comes in scope
    makes_copy(x); // since i32 is Copy, 'x' can be used
                   // afterwards
    println!("{}", x); // valid

    /* Return Value and Scope */

    /* Returning values can also transfer ownership. Consider following code */

    let s1 = gives_ownership(); // 'moving' the return value into s1

    let s2 = String::from("hello"); // 's2' comes into scope

    let s3 = takes_and_gives_back(s2); // s2 is 'moved' into takes_and_gives_back,
                                       // while its return value is moved into s3

    /* s1 = "zenus",
     * s2 = invalid,
     * s3 = "hello"
     * */

    println!("s1 = {}", s1);
    /* println!("s2 = {}", s2); */
 // invalid. compilation error
    println!("s3 = {}", s3);

    /* It's annoying that anything we pass to a function, needs to be passed
     * back if we want to use it again. We might also need the date returning
     * from the function as well. We can used 'tuple', but that is a lot.
     * Example: */

    let s1 = String::from("grazia");
    let (s2, len) = calculate_length(s1);
    println!("The length of {} is {}", s2, len);

    /* Rust provides 'references' to tackle this problem.*/
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("zenus");
    some_string
}

fn takes_and_gives_back(i_string: String) -> String {
    i_string
}

fn calculate_length(string: String) -> (String, usize) {
    let length = string.len();
    (string, length)
}
