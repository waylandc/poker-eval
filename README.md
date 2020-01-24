
Poker hand evaluator using Perfect Hash technique, written in Rust.

Poker hand evaluation has always been an interesting problem and [Cactus Kev](https://www.suffecool.net/poker/evaluator.html) offered a fast implementation of it. Improving on the Cactus Kev method, Paul Senzee brainstormed an even faster method he termed, [Perfect Hash](http://senzee.blogspot.com/2006/06/some-perfect-hash.html).

This is a Rust implementation of the Perfect Hash method of poker hand evaluation. Done as part of my journey in learning the Rust programming language.

While most implementations only work for Texas Holdem, we'll attempt to get support for Omaha also.

