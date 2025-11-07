use stuff::Stuff;

fn main() {
    let s = Stuff::new().with_n(13);
    println!("{}", s.output());
}
