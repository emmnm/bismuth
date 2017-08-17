extern crate bismuth;

use bismuth::LSystem;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;


#[test]
fn test_lsystem_example() {
    // Integration test for example 7 on wiki page
    // https://en.wikipedia.org/wiki/L-system#Example_7:_Fractal_plant
    let mut lsys: LSystem = LSystem::new("X".to_string());
    lsys.add('F');
    lsys.add_constant('+');
    lsys.add_constant('-');
    lsys.add_constant('[');
    lsys.add_constant(']');
    let mut rule: HashMap<char, String> = HashMap::new(); 
    rule.insert('X', "F[−X][X]F[−X]+FX".to_string());
    rule.insert('F', "FF".to_string());
    lsys.push(rule);

    // Iterate 6 times
    for _ in 0..6 {
        lsys = lsys.next();
    }
    
    // Read solution file
    let mut file = File::open("tests/ex7sol.txt").unwrap();
    let mut soln = String::new();
    file.read_to_string(&mut soln).unwrap();
    assert_eq!(lsys.to_string(), soln);
}
