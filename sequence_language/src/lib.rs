use codon::{CodesFor, Codon};
use protein::amino_acid::AminoAcid;

mod codon;

pub mod dna;
mod rna;

mod protein;

mod sequence;

pub use sequence::Sequence;
pub use sequence::Sequenceable;

pub type DNA = crate::dna::nucleobase::Nucleobase;
pub type RNA = crate::rna::nucleobase::Nucleobase;
pub type Protein = crate::protein::amino_acid::AminoAcid;

impl Sequenceable for DNA {
    fn name() -> &'static str {
        "DNA"
    }
}
impl Sequenceable for RNA {
    fn name() -> &'static str {
        "RNA"
    }
}
impl Sequenceable for Protein {
    fn name() -> &'static str {
        "Protein"
    }
}

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
    use crate::{Protein, Sequence, DNA, RNA};

    #[test]
    fn it_works() -> Result<(), anyhow::Error> {
        let sequence = "ACT";

        let dna_sequence: Sequence<DNA> = sequence.try_into()?;

        println!("{}", dna_sequence);

        let rna_sequence: Sequence<RNA> = dna_sequence.into();

        println!("{}", rna_sequence);

        let protein_sequence: Sequence<Protein> =
            rna_sequence.try_into().expect("cant get protein");

        println!("{}", protein_sequence);

        Ok(())
    }
}
