#[macro_export]
macro_rules! stateset {
    () => {
        {
            use std::collections::{BTreeSet};
            let temp_set: BTreeSet<String> = BTreeSet::new();
            temp_set
        }
    };
    ( $( $x:expr ),+ ) => {
        {
            use std::collections::{BTreeSet};
            let mut temp_set: BTreeSet<String> = BTreeSet::new();
            $(
                temp_set.insert($x.to_string());
            )*
            temp_set
        }
    };
}

#[macro_export]
macro_rules! alphabet {
    () => {
        {
            use std::collections::{BTreeSet};
            let alphabet_set: BTreeSet<char> = BTreeSet::new();
            alphabet_set
        }
    };
    ( $( $c:expr ),* ) => {
        {
            use std::collections::{BTreeSet};
            let mut alphabet_set: BTreeSet<char> = BTreeSet::new();
            $(
                alphabet_set.insert($c);
            )*
            alphabet_set
        }
    };
}

#[macro_export]
macro_rules! delta {
    () => {
        {
            use std::collections::{BTreeSet};
            let temp_delta: BTreeSet<(String, char, String)> = BTreeSet::new();
            temp_delta
        }
    };
    ( $( ($s:expr, $c:expr, $ns:expr) ),* ) => {
        {
            use std::collections::{BTreeSet};
            let mut temp_delta: BTreeSet<(String, char, String)> = BTreeSet::new();
            $(
                temp_delta.insert( ($s.to_string(), $c, $ns.to_string()) );
            )*
            temp_delta
        }
    };
}



// Grammar

#[macro_export]
macro_rules! charset {
    () => {
        {
            use std::collections::{BTreeSet};
            let aux: BTreeSet<char> = BTreeSet::new();
            aux
        }
    };
    ( $( $c:expr ),* ) => {
        {
            use std::collections::{BTreeSet};
            let mut aux: BTreeSet<char> = BTreeSet::new();
            $(
                aux.insert($c);
            )*
            aux
        }
    };
}

#[macro_export]
macro_rules! r_productions {
    () => {
        {
            use std::collections::{BTreeSet};
            let aux: BTreeSet<(char, String)> = BTreeSet::new();
            aux
        }
    };
    ( $( ($vn:expr, $chain:expr) ),* ) => {
        {
            use std::collections::{BTreeSet};
            let mut aux: BTreeSet<(char, String)> = BTreeSet::new();
            $(
                aux.insert( ($vn, $chain.to_string()) );
            )*
            aux
        }
    };
}
