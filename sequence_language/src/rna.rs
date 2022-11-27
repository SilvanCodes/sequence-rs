pub mod nucleobase {
    use thiserror::Error;
    pub use Nucleobase::{Adenine, Adenine as A};
    pub use Nucleobase::{Cytosine, Cytosine as C};
    pub use Nucleobase::{Guanine, Guanine as G};
    pub use Nucleobase::{Uracil, Uracil as U};

    #[derive(Debug, Clone, Copy)]
    pub enum Nucleobase {
        Adenine,
        Cytosine,
        Guanine,
        Uracil,
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
                    &Uracil => "U",
                }
            )
        }
    }

    #[derive(Error, Debug)]
    pub enum RnaNucleobaseError {
        #[error("Unexpected character: {0}. Allowed are A, C, U, G.")]
        UnexpectedCharacter(char),
    }

    impl TryFrom<char> for Nucleobase {
        type Error = RnaNucleobaseError;

        fn try_from(value: char) -> Result<Self, Self::Error> {
            match value {
                'A' => Ok(Adenine),
                'C' => Ok(Cytosine),
                'U' => Ok(Uracil),
                'G' => Ok(Guanine),
                _ => Err(RnaNucleobaseError::UnexpectedCharacter(value)),
            }
        }
    }
}
