# Syntaxis (Rust) [![Build Status](https://travis-ci.org/franleplant/syntaxis.rs.svg?branch=master)](https://travis-ci.org/franleplant/syntaxis.rs)

In this repo you will find different algorithms that are used in the context of the course of Syntaxis and Semantics of 
formal Languages.

## How to
Install Rust and Cargo and clone this repo, then just `cargo test`

## TODO

- Change Fsa to Automata
- Improve the internal representation of delta: `BTreeMap<State, BTreeMap<char, BTreeSet<State>>>` that will make delta faster to use
- Read/write automatas to/from file
- automata -> Min automata
- Define a proper data structure for grammars
- Read/Write grammars to/from file
- automata -> grammar
- grammar -> automata
