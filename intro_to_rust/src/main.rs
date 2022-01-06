fn main() {
    println!("Hello, world!");

    // declare and assign variable of type &str
    // note that variable is immutable, meaning I can't redefine it.
    // Keyword 'mut' allows for reassignment

    let name: &str = "ctrlbyte";
    println!("Hello, {}", name);

    let mut name_two: &str = "ctrlbyte";
    name_two = "Husky";

    // Arrays/Vectors/Iterables ?
    let names: Vec<&str> = vec!["Taggart", "Husky", "ctrlbyte", "gopro"];

    // looping
    for n in names {
        // conditionals
        if n == "ctrlbyte" {
            println!("I know you!");
        } else if n == "Husky" {
            println!("I know you too!");
        } else {
            println!("Hello, {}", n);
        }
    }

    let exclaimed: Vec<String> = names.into_iter().map(exclaim).collect();
}

fn exclaim(n: &str) -> String {
    let mut res: String = String::from(n);
    res.push_str("!");
    return res;
}
