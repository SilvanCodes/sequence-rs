pub mod amino_acid {
    use thiserror::Error;
    pub use AminoAcid::{Alanine, Alanine as Ala, Alanine as A};
    pub use AminoAcid::{Arginine, Arginine as Arg, Arginine as R};
    pub use AminoAcid::{Asparagine, Asparagine as Asn, Asparagine as N};
    pub use AminoAcid::{Aspartate, Aspartate as Asp, Aspartate as D};
    pub use AminoAcid::{Cysteine, Cysteine as Cys, Cysteine as C};
    pub use AminoAcid::{Glutamate, Glutamate as Gln, Glutamate as E};
    pub use AminoAcid::{Glutamine, Glutamine as Glu, Glutamine as Q};
    pub use AminoAcid::{Glycine, Glycine as Gly, Glycine as G};
    pub use AminoAcid::{Histidine, Histidine as His, Histidine as H};
    pub use AminoAcid::{Isoleucine, Isoleucine as Ile, Isoleucine as I};
    pub use AminoAcid::{Leucine, Leucine as Leu, Leucine as L};
    pub use AminoAcid::{Lysine, Lysine as Lys, Lysine as K};
    pub use AminoAcid::{Methionine, Methionine as Met, Methionine as M};
    pub use AminoAcid::{Phenylalanine, Phenylalanine as Phe, Phenylalanine as F};
    pub use AminoAcid::{Proline, Proline as Pro, Proline as P};
    pub use AminoAcid::{Serine, Serine as Ser, Serine as S};
    pub use AminoAcid::{Threonine, Threonine as Thr, Threonine as T};
    pub use AminoAcid::{Tryptophan, Tryptophan as Trp, Tryptophan as W};
    pub use AminoAcid::{Tyrosine, Tyrosine as Tyr, Tyrosine as Y};
    pub use AminoAcid::{Valine, Valine as Val, Valine as V};

    pub enum AminoAcid {
        Alanine,
        Arginine,
        Asparagine,
        Aspartate,
        Cysteine,
        Glutamine,
        Glutamate,
        Glycine,
        Histidine,
        Isoleucine,
        Leucine,
        Lysine,
        Methionine,
        Phenylalanine,
        Proline,
        Serine,
        Threonine,
        Tryptophan,
        Tyrosine,
        Valine,
    }

    impl std::fmt::Display for AminoAcid {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "{}",
                match self {
                    &Alanine => "A",
                    &Arginine => "R",
                    &Asparagine => "N",
                    &Aspartate => "D",
                    &Cysteine => "C",
                    &Glutamine => "E",
                    &Glutamate => "Q",
                    &Glycine => "G",
                    &Histidine => "H",
                    &Isoleucine => "I",
                    &Leucine => "L",
                    &Lysine => "K",
                    &Methionine => "M",
                    &Phenylalanine => "F",
                    &Proline => "P",
                    &Serine => "S",
                    &Threonine => "T",
                    &Tryptophan => "W",
                    &Tyrosine => "Y",
                    &Valine => "V",
                }
            )
        }
    }

    #[derive(Error, Debug)]
    pub enum ProteinAminoAcidError {
        #[error("Unexpected character: {0}. Allowed are A, R, N, D, C, E, Q, G, H, I, L, K, M, F, P, S, T, W, Y, V.")]
        UnexpectedCharacter(char),
    }

    impl TryFrom<char> for AminoAcid {
        type Error = ProteinAminoAcidError;

        fn try_from(value: char) -> Result<Self, Self::Error> {
            match value {
                'A' => Ok(Alanine),
                'R' => Ok(Arginine),
                'N' => Ok(Asparagine),
                'D' => Ok(Aspartate),
                'C' => Ok(Cysteine),
                'E' => Ok(Glutamine),
                'Q' => Ok(Glutamate),
                'G' => Ok(Glycine),
                'H' => Ok(Histidine),
                'I' => Ok(Isoleucine),
                'L' => Ok(Leucine),
                'K' => Ok(Lysine),
                'M' => Ok(Methionine),
                'F' => Ok(Phenylalanine),
                'P' => Ok(Proline),
                'S' => Ok(Serine),
                'T' => Ok(Threonine),
                'W' => Ok(Tryptophan),
                'Y' => Ok(Tyrosine),
                'V' => Ok(Valine),
                _ => Err(ProteinAminoAcidError::UnexpectedCharacter(value)),
            }
        }
    }
}
