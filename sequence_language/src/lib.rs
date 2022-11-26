mod amino_acid;
mod codon;

mod dna;
mod rna;

impl From<dna::nucleobase::Nucleobase> for rna::nucleobase::Nucleobase {
    fn from(nucleobase: dna::nucleobase::Nucleobase) -> Self {
        match nucleobase {
            dna::nucleobase::A => rna::nucleobase::A,
            dna::nucleobase::C => rna::nucleobase::C,
            dna::nucleobase::G => rna::nucleobase::G,
            dna::nucleobase::T => rna::nucleobase::U,
        }
    }
    //
}

#[cfg(test)]
mod tests {
    use super::dna;

    #[test]
    fn it_works() {
        let sequence = "ACTG";

        let sequence: dna::sequence::Sequence = sequence.try_into().expect("conversion went wrong");

        println!("Sequence: {}", sequence);
    }
}
