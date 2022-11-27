pub mod nucleobase {
    use thiserror::Error;
    pub use Nucleobase::{Adenine, Adenine as A};
    pub use Nucleobase::{Cytosine, Cytosine as C};
    pub use Nucleobase::{Guanine, Guanine as G};
    pub use Nucleobase::{Thymine, Thymine as T};

    #[derive(Debug, Clone, Copy)]
    pub enum Nucleobase {
        Adenine,
        Cytosine,
        Guanine,
        Thymine,
    }

    impl std::fmt::Display for Nucleobase {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "{}",
                match self {
                    &Adenine => "A",
                    &Cytosine => "C",
                    &Guanine => "G",
                    &Thymine => "T",
                }
            )
        }
    }

    #[derive(Error, Debug)]
    pub enum DnaNucleobaseError {
        #[error("Unexpected character: {0}. Allowed are A, C, T, G.")]
        UnexpectedCharacter(char),
    }

    impl TryFrom<char> for Nucleobase {
        type Error = DnaNucleobaseError;

        fn try_from(value: char) -> Result<Self, Self::Error> {
            match value {
                'A' => Ok(Adenine),
                'C' => Ok(Cytosine),
                'T' => Ok(Thymine),
                'G' => Ok(Guanine),
                _ => Err(DnaNucleobaseError::UnexpectedCharacter(value)),
            }
        }
    }
}
