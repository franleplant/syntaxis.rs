#[derive(PartialEq, Debug)]
enum States {
    S0,
    S1,
    Rejected,
}

enum AcceptedStates {
    S1
}

fn fsa(input: String) {
    let svec = input.chars().collect::<Vec<char>>();
    let len = svec.len();
    let mut state = States::S0;

    for i in 0..len {
        let c = svec[i];
        match (state, c) {
            (States::S0, 'a') => state = States::S1,
            (States::S0, 'b') => state = States::S0,
            (States::S1, 'a') => state = States::S0,
            (States::S1, 'b') => state = States::S1,
            _ =>  state = States::Rejected,
        }

    }

    if state == States::S1 {
        println!("Accepted")
    } else {
        println!("Rejected")
    }

}



#[cfg(test)]
#[test]
fn it_works() {
    println!("\n TEST RESULTS \n");
    fsa(String::from("ab"));
    fsa(String::from("aab"));
    fsa(String::from("ababa"));
}
