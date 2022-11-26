use crate::{
    codon::{self, Codon},
    protein::amino_acid::AminoAcid,
    Protein, DNA, RNA,
};

#[derive(Debug)]
pub struct Sequence<T>(Vec<T>);

impl<T> From<Vec<T>> for Sequence<T> {
    fn from(value: Vec<T>) -> Self {
        Self(value)
    }
}

impl<T> FromIterator<T> for Sequence<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self(iter.into_iter().collect::<Vec<_>>())
    }
}

impl<T> std::ops::Deref for Sequence<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl TryFrom<&str> for Sequence<DNA> {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value
            .chars()
            .enumerate()
            .map(|(pos, chr)| match DNA::try_from(chr) {
                Ok(nucleobase) => Ok(nucleobase),
                Err(_) => return Err("unexpected char at position pos"),
            })
            .collect()
    }
}

impl std::fmt::Display for Sequence<DNA> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DNA sequence: ")?;
        for nucleobase in self.iter() {
            write!(f, "{}", nucleobase)?;
        }
        Ok(())
    }
}

impl TryFrom<&str> for Sequence<RNA> {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value
            .chars()
            .enumerate()
            .map(|(pos, chr)| match RNA::try_from(chr) {
                Ok(nucleobase) => Ok(nucleobase),
                Err(_) => return Err("unexpected char at position pos"),
            })
            .collect()
    }
}

impl std::fmt::Display for Sequence<RNA> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RNA sequence: ")?;
        for nucleobase in self.iter() {
            write!(f, "{}", nucleobase)?;
        }
        Ok(())
    }
}

impl TryFrom<&str> for Sequence<Protein> {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value
            .chars()
            .enumerate()
            .map(|(pos, chr)| match Protein::try_from(chr) {
                Ok(nucleobase) => Ok(nucleobase),
                Err(_) => return Err("unexpected char at position pos"),
            })
            .collect()
    }
}

impl std::fmt::Display for Sequence<Protein> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Protein sequence: ")?;
        for amino_acid in self.iter() {
            write!(f, "{}", amino_acid)?;
        }
        Ok(())
    }
}

impl From<Sequence<DNA>> for Sequence<RNA> {
    fn from(sequence: Sequence<DNA>) -> Self {
        sequence.0.into_iter().map(|dna| dna.into()).collect()
    }
}

impl TryFrom<Sequence<RNA>> for Sequence<Protein> {
    type Error = &'static str;

    fn try_from(value: Sequence<RNA>) -> Result<Self, Self::Error> {
        value
            .0
            .chunks(3)
            .map(|triplet| Codon::try_from(triplet))
            .map(|codon| AminoAcid::try_from(codon?))
            .collect()
    }
}
