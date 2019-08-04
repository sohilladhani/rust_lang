fn main() {
    let width = 50;
    let height = 30;

    println!("area of rectangle is {} square pixels", area(width, height));

    /* Although the parameters of 'area' function are related, it is not
     * expressed anywhere in our program. One way to express it is using
     * tuples. */

    /* Refactoring with Tuples */

    let rect1 = (50, 30);

    println!("area of rectangle is {} square pixels", area_tuple(rect1));

    /* Although the tuple has a bit of structure, the version is still less
     * clear as tuples don't name their elements, so the calculation has
     * become more confusing since we have to use indices to access the
     * elements of the tuple. We have to keep track of the indices of height
     * and width. We can use a struct to add more meaning. */

    let rect1 = Rectangle {
        width: 30,
        height: 30,
    };

    println!("area of rectangle is {} square pixels", area_struct(&rect1));

    /* Adding useful functionality with derived traits */

    /* Following code gives errrors
     * println!("area is {}", rect1);
     *
     * error[E0277]: `Rectangle` doesn't implement `std::fmt::Display`
     * note: in format strings you may be able to use `{:?}` (or {:#?} for
     * pretty-print) instead
     *
     * The struct has not implemented the trait 'Display'. Since primitive
     * types implement Display trait by defaut, println! prints it as it is.
     *
     *
     * Rust includes a functionality to print out debugging information but
     * we have to explicitly opt in to make that functionality available to
     * the struct. We do this by adding the annotation #[derive(Debug)]. We
     * have to use {:?} formatter for a struct.
     */
    println!("Struct content is {:?}", rect1);

    /* The above code does not print a pretty code. To pretty-print the struct
     * we use {:#?} formatter instead of {:?} formatter */

    println!("Struct content is {:#?}", rect1);

    /* Rust provides a number of traits for us to use with the 'derive'
     * annotation that can add useful behavior to our custom types.
     *
     * Here the area function is very specific: it only computes the area of
     * rectangles. How can we tie this behavior to the struct rather than the
     * area function, so that it works any other type?
     *
     * We can change the 'area function' to 'area method'. (Next chapter)
     * */
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn area_struct(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}

fn area_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}
