use std::collections::{BTreeSet, BTreeMap};
use automata::{State, Alphabet, StateSet, Char};


//TODO: use this as underlying type for Stack and States
pub type Symbol = String;
pub type SymbolSet = BTreeSet<Symbol>
//TODO:

// This one is the single underlying abstraction
stringset!
// This ones are simple alyiases that call stringset
symbolset!
stateset!

pub type PDADeltaMapKey = (State, Char, Symbol);
pub type PDADeltaMapValue = (State, SymbolSet);
pub type PDADeltaMap = BTreeMap<PDADeltaMapKey, PDADeltaMapValue>;






#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PDA {
    pub k: StateSet,
    pub alphabet: Alphabet,
    pub stack_alphabet: SymbolSet,
    pub q0: State,
    pub z0: Symbol,
    pub f: StateSet,
    pub delta: PDADeltaMap,

    state: State,
    stack: Symbol
}
