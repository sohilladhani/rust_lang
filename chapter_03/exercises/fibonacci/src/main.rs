use std::io;

fn nth_fibonacci(n: usize) -> usize {
    if n <= 1 {
        return n;
    } else {
        return nth_fibonacci(n-1) + nth_fibonacci(n-2);
    }
}

fn main() {
    loop {
        println!("Please enter n: ");

        let mut n = String::new();

        io::stdin().read_line(&mut n).expect("Failed to read line");

        let n: usize = match n.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("nth fibonacci: {}", nth_fibonacci(n));

        println!("Keep going? (y/n)[n]:");
        let mut ans = String::new();
        io::stdin()
            .read_line(&mut ans)
            .expect("Failed to read line");

        match ans.as_str().trim() {
            "y" | "Y" => {
                continue;
            }
            _ => {
                break;
            }
        };
    }
}
