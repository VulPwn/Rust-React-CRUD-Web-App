fn main() {
    let mut date = "11/1/1960";
    let cut = date.split("/");
    for x in cut {
        println!("{}", x.parse::<i32>().unwrap());
    }
}
