use rski::CombinatoryTerm;

fn main() {
    let c = CombinatoryTerm::new("I(SK)").unwrap();
    println!("parsed {:?}", c);
    println!("result {:?}", c.evaluate());
}
