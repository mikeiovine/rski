use rski::CombinatoryTerm;

fn main() {
    let mut c = CombinatoryTerm::new("S(S(KS)(S(KK)(S(KS)K)))(S(K(S(SKK)))K)SKS").unwrap();
    c.evaluate();
    println!("End {}", c);
}
