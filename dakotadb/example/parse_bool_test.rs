fn main() {
    let test: &str = "false";
    println!("{}", test.to_string().parse::<bool>().unwrap());
    println!("{}", !test.to_string().parse::<bool>().unwrap());
}
