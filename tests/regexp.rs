extern crate syntaxis;

use syntaxis::regexp::re1;
use syntaxis::regexp::re2;

#[test]
fn regexp1() {
    assert!(re1("hola".to_string(), "hola").is_ok());
    assert!(re1("hola+".to_string(), "holaaaaaa").is_ok());
    assert!(re1("(ho)+la+".to_string(), "hoholaaaaaa").is_ok());

    assert!(re2("hola".to_string(), "hola").is_ok());
    assert!(re2("hola+".to_string(), "holaaaaaa").is_ok());
    assert!(re2("(ho)+la+".to_string(), "hoholaaaaaa").is_ok());
}
