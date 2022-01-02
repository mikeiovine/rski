use rski::CombinatoryTerm;

fn main() {
    let c = CombinatoryTerm::new("S(S(KS)(S(KK)(S(KS)K)))(S(K(S(SKK)))K)SKS").unwrap();
    println!("starting with {}", c);
    println!("result {}", c.evaluate());
}
