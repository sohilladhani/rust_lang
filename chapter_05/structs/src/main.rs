/* Structs are similar to tuples. To define a struct we use the keyword
 * 'struct'. Inside the curly brackets, we define the names and types of
 * the pieces of data, which we call 'fields'. */

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    /* Creating an instance of the struct */

    let user1 = User {
        email: String::from("abc@xyz.com"),
        username: String::from("abc"),
        active: true,
        sign_in_count: 1,
    };

    /* We can use dot notation to get specific value from the struct instance */

    println!(
        "username is {}, email is {}, active?: {}, sign in count is {}",
        user1.username, user1.email, user1.active, user1.sign_in_count
    );

    /* Creating a mutable instance of the struct to edit a field. The whole
     * instance of the struct has to be mutable, and not indivual fields. */

    let mut user1 = User {
        email: String::from("abc@xyz.com"),
        username: String::from("abc"),
        active: true,
        sign_in_count: 1,
    };

    /* We can use dot notation to get specific value from the struct instance */

    user1.email = String::from("new_email@abc.com");
    println!(
        "username is {}, email is {}, active?: {}, sign in count is {}",
        user1.username, user1.email, user1.active, user1.sign_in_count
    );

    /* One can also create a function to return the struct instance as an
     * expression. */

    let user_email = String::from("aaa@bbb.com");
    let user_username = String::from("aaabbb");
    let user2 = build_user(user_email, user_username);
    println!(
        "username is {}, email is {}, active?: {}, sign in count is {}",
        user2.username, user2.email, user2.active, user2.sign_in_count
    );

    /* Creating struct instances from other instances using struct update syntax */

    /* Creating user3 from user1 */
    let user3 = User {
        // we want email and username to be different for user3. rest of the
        // fields are same as user1
        email: String::from("user3"),
        username: String::from("user3@newuser.com"),
        ..user1
    };

    /* Tuple structs */

    /* When you don't want to name the individual fileds with just types*/

    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    /* Although black and origin have i32 as individual types, they are not
     * same since they are instance of different tuple structs. */

    /* Unit-Like structs without any fields */

    /* They are structs without any fields. They behave similar to (). They
     * are useful in situations in which one needs to implement a 'trait'
     * on some type but don't have any data that you want to store in  the
     * type itself. */

    /* Ownership of Struct Data */

    /* The User struct contains String type rather than '&str' type for username
     * and email fields, since the instances of the struct should own data as
     * long as the entire struct is valid. It's possible to use references to
     * data owned by something else, but to do so requires the use of lifetimes.*/
}

fn build_user(email: String, username: String) -> User {
    User {
        //email: email,
        //shorthand since parameter name and field name are same
        email,
        //username: username,
        username,
        active: false,
        sign_in_count: 2,
    }
}
