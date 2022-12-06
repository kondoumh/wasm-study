fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s)
        .expect("input error at read_line()");
    println!("input: {}", s);
}
