# rski
This is a SKI combinatory logic engine implemented in Rust.

The combinators follow the following set of evaluation rules:
```
Sxyz = xz(yz)
Kxy = x
Ix = x
```

The `I` combinator can be thought of as syntatic sugar since
the identity function can actually be implemented with the
`S` and `K` combinators alone:
```
SKKx = Kx(Kx) = x
```

It can be shown that that this computational model is Turing complete.
See [the Wikipedia article](https://en.wikipedia.org/wiki/SKI_combinator_calculus)
on the subject for more details.

## Usage
A REPL is provided via `cargo run`.
```
> SSSSSSSSSSSSSS
SS(SS(SS(SS(SS(SS(SS))))))
> S(K(SK)S)
S(SK)`
```

The binary alternatively accepts a file, e.g. `cargo run -- my_combinator.txt`.
In this mode the engine prints all out intermediate steps:
```
starting combinator: S(K(S))(S(K(S(K(S))))(S(K(K))))(S(S(K(S))(K))(S(K)))(S(S(K(S))(K))(S(K)))SK
(K(S))(S(S(K(S))(K))(S(K)))((S(K(S(K(S))))(S(K(K))))(S(S(K(S))(K))(S(K))))(S(S(K(S))(K))(S(K)))SK
(S)((S(K(S(K(S))))(S(K(K))))(S(S(K(S))(K))(S(K))))(S(S(K(S))(K))(S(K)))SK
((S(K(S(K(S))))(S(K(K))))(S(S(K(S))(K))(S(K))))S((S(S(K(S))(K))(S(K)))S)K
(K(S(K(S))))(S(S(K(S))(K))(S(K)))((S(K(K)))(S(S(K(S))(K))(S(K))))S((S(S(K(S))(K))(S(K)))S)K
(S(K(S)))((S(K(K)))(S(S(K(S))(K))(S(K))))S((S(S(K(S))(K))(S(K)))S)K
(K(S))S(((S(K(K)))(S(S(K(S))(K))(S(K))))S)((S(S(K(S))(K))(S(K)))S)K
(S)(((S(K(K)))(S(S(K(S))(K))(S(K))))S)((S(S(K(S))(K))(S(K)))S)K
(((S(K(K)))(S(S(K(S))(K))(S(K))))S)K(((S(S(K(S))(K))(S(K)))S)K)
(K(K))S((S(S(K(S))(K))(S(K)))S)K(((S(S(K(S))(K))(S(K)))S)K)
(K)((S(S(K(S))(K))(S(K)))S)K(((S(S(K(S))(K))(S(K)))S)K)
((S(S(K(S))(K))(S(K)))S)(((S(S(K(S))(K))(S(K)))S)K)
(S(K(S))(K))S((S(K))S)(((S(S(K(S))(K))(S(K)))S)K)
(K(S))S((K)S)((S(K))S)(((S(S(K(S))(K))(S(K)))S)K)
(S)((K)S)((S(K))S)(((S(S(K(S))(K))(S(K)))S)K)
((K)S)(((S(S(K(S))(K))(S(K)))S)K)(((S(K))S)(((S(S(K(S))(K))(S(K)))S)K))
S(((S(K))S)(((S(S(K(S))(K))(S(K)))S)K))
S((K)(((S(S(K(S))(K))(S(K)))S)K)(S(((S(S(K(S))(K))(S(K)))S)K)))
S((((S(S(K(S))(K))(S(K)))S)K))
S((S(K(S))(K))S((S(K))S)K)
S((K(S))S((K)S)((S(K))S)K)
S((S)((K)S)((S(K))S)K)
S(((K)S)K(((S(K))S)K))
S(S(((S(K))S)K))
S(S((K)K(SK)))
S(S(K))
derived S(S(K)) after 25 steps
```
