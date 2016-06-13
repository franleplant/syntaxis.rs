#[macro_use]
extern crate syntaxis;

use syntaxis::{
    M,
    GR,
    NonTerminalSet,
    TerminalSet,
    NonTerminal,
    RegularProductions,
    gr_to_afndl,
    minify,
    afndl_to_afd
};

#[test]
fn grammar_to_automata_min() {

    let vn: NonTerminalSet = charset!('S');
    let vt: TerminalSet = charset!('a');
    let q0: NonTerminal = 'S';
    let productions: RegularProductions = r_productions!(
        ('S', "aS"),
        ('S', "a")
    );

    let gr: GR = GR::new(vt, vn, productions, q0);
    let m: M = gr_to_afndl(&gr);
    let m: M = afndl_to_afd(&m);
    let m: M = minify(&m);
    println!("{:?}", m);

    assert!(true);
}
