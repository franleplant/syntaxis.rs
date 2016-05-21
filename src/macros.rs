//use std::collections::{BTreeSet, BTreeMap};

macro_rules! stateset {
    () => {
        {
            let temp_set: BTreeSet<String> = BTreeSet::new();
            temp_set
        }
    };
    ( $( $x:expr ),+ ) => {
        {
            let mut temp_set: BTreeSet<String> = BTreeSet::new();
            $(
                temp_set.insert($x.to_string());
            )*
            temp_set
        }
    };
}

macro_rules! alphabet {
    () => {
        {
            let alphabet_set: BTreeSet<char> = BTreeSet::new();
            alphabet_set
        }
    };
    ( $( $c:expr ),* ) => {
        {
            let mut alphabet_set: BTreeSet<char> = BTreeSet::new();
            $(
                alphabet_set.insert($c);
            )*
            alphabet_set
        }
    };
}

macro_rules! delta {
    () => {
        {
            let temp_delta: BTreeSet<(String, char, String)> = BTreeSet::new();
            temp_delta
        }
    };
    ( $( ($s:expr, $c:expr, $ns:expr) ),* ) => {
        {
            let mut temp_delta: BTreeSet<(String, char, String)> = BTreeSet::new();
            $(
                temp_delta.insert( ($s.to_string(), $c, $ns.to_string()) );
            )*
            temp_delta
        }
    };
}
