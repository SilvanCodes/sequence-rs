pub mod nucleobase {
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

    impl TryFrom<char> for Nucleobase {
        type Error = &'static str;

        fn try_from(value: char) -> Result<Self, Self::Error> {
            match value {
                'A' => Ok(Adenine),
                'C' => Ok(Cytosine),
                'T' => Ok(Thymine),
                'G' => Ok(Guanine),
                _unknown => Err("got unexpected char {}, allowed are: A, C, T, G"),
            }
        }
    }
}
