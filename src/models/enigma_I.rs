use std::collections::HashMap;

use crate::enigma::{get_rotor, EnigmaMachine};

impl EnigmaMachine {
    pub fn enigma_I() -> Self {
        EnigmaMachine {
            plugboard: HashMap::new(),
            rotors: vec![rotor_1(), rotor_2(), rotor_3()],
        }
    }
    pub fn set_plugboard(mut self, input: char, output: char) -> Self {
        //should not allow to set an input char equal to an output char
        self.plugboard.insert(input, output);
        self
    }
}

fn rotor_1() -> HashMap<char, char> {
    get_rotor("EKMFLGDQVZNTOWYHXUSPAIBRCJ")
}

fn rotor_2() -> HashMap<char, char> {
    get_rotor("AJDKSIRUXBLHWTMCQGZNPYFVOE")
}

fn rotor_3() -> HashMap<char, char> {
    get_rotor("BDFHJLCPRTXVZNYEIWGAKMUSQO")
}
