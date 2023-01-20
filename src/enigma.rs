use std::collections::HashMap;

pub struct EnigmaMachine {
    pub plugboard: HashMap<char, char>,
    pub rotors: Vec<HashMap<char, char>>,
}

impl EnigmaMachine {
    fn set_plug(mut self, input: char, output: char) {
        self.plugboard.insert(input, output);
    }

    pub fn decode(self, input: &str) -> &str {
        let after_plugboard = decode_plugboard(input);
        let after_rotors = decode_rotors(after_plugboard);
        let after_reflected = decode_plugboard(after_rotors);

        after_reflected
    }

    pub fn encode(self, input: &str) -> &str {
        todo!()
    }
}

fn decode_rotors(after_plugboard: &str) -> &str {
    todo!()
}

fn decode_plugboard(input: &str) -> &str {
    todo!()
}

pub fn get_rotor(output: &str) -> HashMap<char, char> {
    alphabet().into_iter().zip(output.chars()).collect()
}

fn alphabet() -> Vec<char> {
    ('A'..='Z').into_iter().collect()
}
