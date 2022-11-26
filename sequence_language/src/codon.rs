use super::amino_acid::*;
use super::rna::nucleobase::{Nucleobase, A, C, G, U};
struct Codon([Nucleobase; 3]);

pub enum CodesFor {
    AminoAcid(AminoAcid),
    Stop,
}

impl From<Codon> for CodesFor {
    fn from(Codon([first, second, third]): Codon) -> Self {
        match first {
            U => match second {
                U => match third {
                    U | C => CodesFor::AminoAcid(Phenylalanine),
                    A | G => CodesFor::AminoAcid(Leucine),
                },
                C => match third {
                    U | C | A | G => CodesFor::AminoAcid(Serine),
                },
                A => match third {
                    U | C => CodesFor::AminoAcid(Tyrosine),
                    A | G => CodesFor::Stop,
                },
                G => match third {
                    U | C => CodesFor::AminoAcid(Cysteine),
                    A => CodesFor::Stop,
                    G => CodesFor::AminoAcid(Tryptophan),
                },
            },
            C => match second {
                U => match third {
                    U | C | A | G => CodesFor::AminoAcid(Leucine),
                },
                C => match third {
                    U | C | A | G => CodesFor::AminoAcid(Proline),
                },
                A => match third {
                    U | C => CodesFor::AminoAcid(Histidine),
                    A | G => CodesFor::AminoAcid(Glutamine),
                },
                G => match third {
                    U | C | A | G => CodesFor::AminoAcid(Arginine),
                },
            },
            A => match second {
                U => match third {
                    U | C | A => CodesFor::AminoAcid(Isoleucine),
                    G => CodesFor::AminoAcid(Methionine),
                },
                C => match third {
                    U | C | A | G => CodesFor::AminoAcid(Threonine),
                },
                A => match third {
                    U | C => CodesFor::AminoAcid(Asparagine),
                    A | G => CodesFor::AminoAcid(Lysine),
                },
                G => match third {
                    U | C => CodesFor::AminoAcid(Serine),
                    A | G => CodesFor::AminoAcid(Arginine),
                },
            },
            G => match second {
                U => match third {
                    U | C | A | G => CodesFor::AminoAcid(Valine),
                },
                C => match third {
                    U | C | A | G => CodesFor::AminoAcid(Alanine),
                },
                A => match third {
                    U | C => CodesFor::AminoAcid(Aspartate),
                    A | G => CodesFor::AminoAcid(Glutamate),
                },
                G => match third {
                    U | C | A | G => CodesFor::AminoAcid(Glycine),
                },
            },
        }
    }
}
