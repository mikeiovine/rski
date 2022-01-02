use rski::CombinatoryTerm;

fn main() {
    let verbose = true;
    let mut c = CombinatoryTerm::new("(S(S(SS))SSS)", verbose).unwrap();
    c.evaluate();
}
