use rski::{CombinatoryTerm, Printer};

fn main() {
    let mut c = CombinatoryTerm::new("S(S(KS)(S(KK)(S(KS)K)))(S(K(S(SKK)))K)SKS").unwrap();
    c.attach(Box::new(Printer::new()));
    c.evaluate();

    let mut c = CombinatoryTerm::new("S(S(SS))SSS").unwrap();
    c.attach(Box::new(Printer::new()));
    c.evaluate();

    let mut c = CombinatoryTerm::new("((((S))))").unwrap();
    c.attach(Box::new(Printer::new()));
    c.evaluate();
}
