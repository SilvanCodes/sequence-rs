pub mod nucleobase {
    pub use Nucleobase::Adenine;
    pub use Nucleobase::Cytosine;
    pub use Nucleobase::Guanine;
    pub use Nucleobase::Thymine;

    pub use Nucleobase::Adenine as A;
    pub use Nucleobase::Cytosine as C;
    pub use Nucleobase::Guanine as G;
    pub use Nucleobase::Thymine as T;

    #[derive(Debug)]
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
                'A' => Ok(Nucleobase::Adenine),
                'C' => Ok(Nucleobase::Cytosine),
                'T' => Ok(Nucleobase::Thymine),
                'G' => Ok(Nucleobase::Guanine),
                _unknown => Err("got unexpected char {}, allowed are: A, C, T, G"),
            }
        }
    }
}

pub mod sequence {
    #[derive(Debug)]
    pub struct Sequence(Vec<super::nucleobase::Nucleobase>);

    impl std::fmt::Display for Sequence {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "DNA sequence: ")?;
            for nucleobase in self.iter() {
                write!(f, "{}", nucleobase)?;
            }
            Ok(())
        }
    }

    impl From<Vec<super::nucleobase::Nucleobase>> for Sequence {
        fn from(value: Vec<super::nucleobase::Nucleobase>) -> Self {
            Self(value)
        }
    }

    impl FromIterator<super::nucleobase::Nucleobase> for Sequence {
        fn from_iter<T: IntoIterator<Item = super::nucleobase::Nucleobase>>(iter: T) -> Self {
            Self(iter.into_iter().collect::<Vec<_>>())
        }
    }

    impl std::ops::Deref for Sequence {
        type Target = Vec<super::nucleobase::Nucleobase>;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl TryFrom<&str> for Sequence {
        type Error = &'static str;

        fn try_from(value: &str) -> Result<Self, Self::Error> {
            value
                .chars()
                .enumerate()
                .map(
                    |(pos, chr)| match super::nucleobase::Nucleobase::try_from(chr) {
                        Ok(nucleobase) => Ok(nucleobase),
                        Err(_) => return Err("unexpected char at position pos"),
                    },
                )
                .collect()
        }
    }
}
