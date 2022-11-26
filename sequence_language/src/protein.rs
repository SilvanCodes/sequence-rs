pub mod amino_acid {
    pub use AminoAcid::Alanine;
    pub use AminoAcid::Arginine;
    pub use AminoAcid::Asparagine;
    pub use AminoAcid::Aspartate;
    pub use AminoAcid::Cysteine;
    pub use AminoAcid::Glutamate;
    pub use AminoAcid::Glutamine;
    pub use AminoAcid::Glycine;
    pub use AminoAcid::Histidine;
    pub use AminoAcid::Isoleucine;
    pub use AminoAcid::Leucine;
    pub use AminoAcid::Lysine;
    pub use AminoAcid::Methionine;
    pub use AminoAcid::Phenylalanine;
    pub use AminoAcid::Proline;
    pub use AminoAcid::Serine;
    pub use AminoAcid::Threonine;
    pub use AminoAcid::Tryptophan;
    pub use AminoAcid::Tyrosine;
    pub use AminoAcid::Valine;

    pub use Alanine as Ala;
    pub use Arginine as Arg;
    pub use Asparagine as Asn;
    pub use Aspartate as Asp;
    pub use Cysteine as Cys;
    pub use Glutamate as Gln;
    pub use Glutamine as Glu;
    pub use Glycine as Gly;
    pub use Histidine as His;
    pub use Isoleucine as Ile;
    pub use Leucine as Leu;
    pub use Lysine as Lys;
    pub use Methionine as Met;
    pub use Phenylalanine as Phe;
    pub use Proline as Pro;
    pub use Serine as Ser;
    pub use Threonine as Thr;
    pub use Tryptophan as Trp;
    pub use Tyrosine as Tyr;
    pub use Valine as Val;

    pub use Alanine as A;
    pub use Arginine as R;
    pub use Asparagine as N;
    pub use Aspartate as D;
    pub use Cysteine as C;
    pub use Glutamate as E;
    pub use Glutamine as Q;
    pub use Glycine as G;
    pub use Histidine as H;
    pub use Isoleucine as I;
    pub use Leucine as L;
    pub use Lysine as K;
    pub use Methionine as M;
    pub use Phenylalanine as F;
    pub use Proline as P;
    pub use Serine as S;
    pub use Threonine as T;
    pub use Tryptophan as W;
    pub use Tyrosine as Y;
    pub use Valine as V;

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

    impl TryFrom<char> for AminoAcid {
        type Error = &'static str;

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

                _unknown => Err("got unexpected char {}, allowed are: A, R, N, D, C, E, Q, G, H, I, L, K, M, F, P, S, T, W, Y, V"),
            }
        }
    }
}
