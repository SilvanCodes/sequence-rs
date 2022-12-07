use std::marker::PhantomData;

use thiserror::Error;

use crate::{codon::Codon, protein::amino_acid::AminoAcid, Protein, DNA, RNA};

pub trait Sequenceable: std::fmt::Display {
    fn name() -> &'static str;
}

#[derive(Debug)]
pub struct Sequence<T: Sequenceable, S = Vec<T>> {
    made_from: PhantomData<T>,
    data: S,
}

impl<T: Sequenceable> Sequence<T> {
    pub fn new(data: Vec<T>) -> Self {
        Self {
            made_from: PhantomData,
            data,
        }
    }
}

impl<T: Sequenceable> From<Vec<T>> for Sequence<T> {
    fn from(value: Vec<T>) -> Self {
        Self::new(value)
    }
}

impl<T: Sequenceable> FromIterator<T> for Sequence<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self::new(iter.into_iter().collect::<Vec<_>>())
    }
}

impl<T: Sequenceable, S> std::ops::Deref for Sequence<T, S> {
    type Target = S;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

#[derive(Error, Debug)]
pub enum SequenceError<T> {
    #[error("Unexpected character in sequence at position: {position}")]
    UnexpectedCharacter { source: T, position: usize },
}

impl<T: TryFrom<char> + Sequenceable> TryFrom<&str> for Sequence<T> {
    type Error = SequenceError<<T as TryFrom<char>>::Error>;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value
            .chars()
            .enumerate()
            .map(|(position, character)| match T::try_from(character) {
                Ok(sequence_item) => Ok(sequence_item),
                Err(source) => return Err(SequenceError::UnexpectedCharacter { position, source }),
            })
            .collect()
    }
}

impl<S: Sequenceable> std::fmt::Display for Sequence<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} sequence: ", S::name())?;
        for nucleobase in self.iter() {
            write!(f, "{}", nucleobase)?;
        }
        Ok(())
    }
}

impl From<Sequence<DNA>> for Sequence<RNA> {
    fn from(sequence: Sequence<DNA>) -> Self {
        sequence.data.into_iter().map(|dna| dna.into()).collect()
    }
}

impl TryFrom<Sequence<RNA>> for Sequence<Protein> {
    type Error = &'static str;

    fn try_from(value: Sequence<RNA>) -> Result<Self, Self::Error> {
        value
            .data
            .chunks(3)
            .map(|triplet| Codon::try_from(triplet))
            .map(|codon| AminoAcid::try_from(codon?))
            .collect()
    }
}
