/* Enums (or enumerations) allow us to define a type by enumerating its
 * possible values. Enum values can only be one of the variants.
 *
 * Example: IP adresses. An IP address could be of version 4 or version 6
 * See the below example code
 *
 * */

enum IpAddrKind {
    V4,
    V6,
}

enum IpAddrEnum {
    V4(String),
    V6(String),
}

fn main() {
    /* We can create instances of each of the 2 variants of IpAddrKind */

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    /* Variants of the enum are namespaced under its indentifier. We use
     * a double colon to separate the two. We can write a function that
     * takes any IpAddrKind. Check 'route' function below. */

    /* We can call 'route' function with either variant. */

    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    /* We can give meaning to a IpAddrKind enum by using a struct 'IpAddr'
     * like below
     *
     * */

    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    /* We can represent the same concept in a more concise way just using an
     * enum, rather than an enum inside a struct, by putting data directly
     * into each variant. See the definition of IpAddrEnum above. */

    let home = IpAddrEnum::V4(String::from("127.0.0.1"));
    let loopback = IpAddrEnum::V6(String::from("::1"));

    /* Like depicted above, we can attach data to each variant of the enum
     * directly, so there is no need for an extra struct.
     *
     * There is another advantage to using an enum rather than a struct;
     * each variant can have different types and amounts of associated data.
     *
     * For example, V4 IP address will always have 4 numeric components that
     * will have values between 0 and 255. If we wanted to store V4 addresses
     * as 4 'u8' values but still express V6 address as one String value, we
     * wouldn't be able to with a struct.
     * */

    enum IpAddrVariant {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let home = IpAddrVariant::V4(127, 0, 0, 1);
    let loopback = IpAddrVariant::V6(String::from("::1"));

    /* The standard library has a definition for IP address we can use. It
     * uses 2 structs inside an enum like below.
     *
     * struct Ipv4Addr { //code }
     *
     * struct Ipv6Addr { //code }
     *
     * enum IpAddr { V4(Ipv4Addr), V6(Ipv6Addr), }
     *
     * */

    /* Another example enum each one storing different amounts and types of
     * values.
     * */

    enum Message {
        Quit,                       // No data associated
        Move { x: i32, y: i32 },    // includes an anonymous struct inside it
        Write(String),              // includes a single String
        ChangeColor(i32, i32, i32), // includes three i32 values
    }

    /* Defining an enum with variants such as above is similar to defining
     * different kinds of struct definitions, except the enum doesn't use the
     * 'struct' keyword and all the variants are grouped together under the
     * 'Message' type. The following structs could hold the same data the
     * preceding enum variants hold:
     *
     * */

    struct QuitMessage; // unit struct
    struct MoveMessage {
        x: i32,
        y: i32,
    }
    struct WriteMessage(String); // tuple struct
    struct ChangeColorMessage(i32, i32, i32); // tuple struct
}

fn route(ip_kind: IpAddrKind) {}
