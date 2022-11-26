use codon::{CodesFor, Codon};
use protein::amino_acid::AminoAcid;

mod codon;

mod dna;
mod rna;

mod protein;

type DNA = crate::dna::nucleobase::Nucleobase;
type RNA = crate::rna::nucleobase::Nucleobase;
type Protein = crate::protein::amino_acid::AminoAcid;

mod sequence;

impl From<DNA> for RNA {
    fn from(nucleobase: DNA) -> Self {
        match nucleobase {
            dna::nucleobase::A => rna::nucleobase::A,
            dna::nucleobase::C => rna::nucleobase::C,
            dna::nucleobase::G => rna::nucleobase::G,
            dna::nucleobase::T => rna::nucleobase::U,
        }
    }
}

impl TryFrom<Codon> for AminoAcid {
    type Error = &'static str;

    fn try_from(value: Codon) -> Result<Self, Self::Error> {
        let codes_for: CodesFor = value.into();

        match codes_for {
            CodesFor::AminoAcid(amino_acid) => Ok(amino_acid),
            CodesFor::Stop => Err("encountered unexpectes stop codon at {}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::sequence;

    #[test]
    fn it_works() {
        let sequence = "ACT";

        let dna_sequence: sequence::Sequence<crate::dna::nucleobase::Nucleobase> =
            sequence.try_into().expect("conversion went wrong");

        println!("{}", dna_sequence);

        let rna_sequence: sequence::Sequence<crate::rna::nucleobase::Nucleobase> =
            dna_sequence.into();

        println!("{}", rna_sequence);

        let protein_sequence: sequence::Sequence<crate::protein::amino_acid::AminoAcid> =
            rna_sequence.try_into().expect("cant get protein");

        println!("{}", protein_sequence);
    }
}
