#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    /* Method syntax */

    /* Methods are similar to functions, in that they have 'fn' keyword,
    * parameters and return value.
    *
    * Methods are different to functions, in that they are defined within
    * the context of struct (or an enum or a trait object), and their first
    * parameter is always 'self', which corresponds to the instance of the
    * struct the method is being called on. See the above code for Rectangle
    * struct and method 'area'.
       #[derive(Debug)]
       struct Rectangle {
           width: u32,
           height: u32
       }

       // &self = immutable reference to the instance of struct Rectangle
       impl Rectangle {
           fn area(&self) -> u32 {
               self.width * self.height
           }
       }
    *
    * */

    let rect1 = Rectangle {
        width: 50,
        height: 30,
    };

    println!("rectangle area is {}", rect1.area());

    /* The main benefit of using methods instead of functions, in addition
     * to using method syntax and not having to repeat the type of 'self'
     * in every method's signature, is for organization.
     *
     * In our example, we have put all the thinge we can do with an instance
     * of a type in one 'impl' block rather than making future users of our
     * code search for capabiliites of 'Rectangle' in various places in the
     * library we provide. */

    /* Methods with more parameters */

    /* We have defined a new method 'can_hold', which takes an instance of a
     * Rectangle and returns true if the second Rectangle can fit completely
     * within self; otherwise it returns false. */

    let rect2 = Rectangle {
        width: 40,
        height: 20,
    };
    let rect3 = Rectangle {
        width: 50,
        height: 60,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    /* Associated functions */

    /* These are functions associated within 'impl' blocks that don't take
     * 'self' as a parameter. These are called associated functions because
     * they are associated with the struct. Thye are still functions, and
     * not methods. Associated functions do not have the instance of the
     * struct to work with.
     *
     * They are often used as 'constructors' that will return a new instance
     * of the struct. One example is to have a 'square' function which only
     * takes 1 parameter for both width and height so that one does not need
     * to provide same value twice.
     *
     * The syntax of associated functions is:
     * <struct>::<associated_function>
     *
     * Example, the 'square' function above. And the calling of the function
     * below.
     *
     * */

    let size = 30;
    let rect4 = Rectangle::square(30);
    println!(
        "Square of size {} is {:?} with area {}",
        size,
        rect4,
        rect4.area()
    );

    /* One can have multiple 'impl' blocks for a sruct */
}
