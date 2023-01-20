mod enigma;
mod models;

#[cfg(test)]
mod tests {
    use crate::enigma::EnigmaMachine;

    #[test]
    fn decode_enigma_1() {
        let enigma = EnigmaMachine::enigma_I();

        let decoded = enigma.decode("");

        assert_eq!(decoded, "");
    }
}
