use rski::CombinatoryTerm;

fn main() {
    let verbose = true;
    let mut c = CombinatoryTerm::new("S(S(KS)(S(KK)(S(KS)K)))(S(K(S(SKK)))K)SKS", verbose).unwrap();
    c.evaluate();
}
