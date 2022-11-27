pub mod nucleobase {
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

    impl TryFrom<char> for Nucleobase {
        type Error = &'static str;

        fn try_from(value: char) -> Result<Self, Self::Error> {
            match value {
                'A' => Ok(Adenine),
                'C' => Ok(Cytosine),
                'U' => Ok(Uracil),
                'G' => Ok(Guanine),
                _unknown => Err("got unexpected char {}, allowed are: A, C, T, G"),
            }
        }
    }
}
