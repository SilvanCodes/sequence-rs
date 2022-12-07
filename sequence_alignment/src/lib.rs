use std::marker::PhantomData;

use sequence_language::{Sequence, Sequenceable};

pub trait AlignmentType {}
pub struct Global;
impl AlignmentType for Global {}

struct Local;

impl AlignmentType for Local {}

#[derive(Debug)]
enum AlignedPosition {
    MatchedIn(Vec<usize>),
    DeletedIn(Vec<usize>),
}

pub struct Alignment<T: AlignmentType, S: Sequenceable> {
    alignment_type: PhantomData<T>,
    sequences: Vec<Sequence<S>>,
    alignment: Vec<AlignedPosition>,
}

impl<T: AlignmentType, S: Sequenceable> Alignment<T, S> {
    // TODO:
    // type-level builder pattern as interface needleman-wunsch
    //
    // fn add_sequence
    // fn set_substitution_matrix
    // fn set gap_cost
    // fn align
}

impl<T: AlignmentType, S: Sequenceable> std::fmt::Display for Alignment<T, S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let write_sequence_according_to_alignment =
            |sequence_index: usize, f: &mut std::fmt::Formatter<'_>| -> std::fmt::Result {
                let mut current_sequence_position = 0;
                for position in &self.alignment {
                    match position {
                        AlignedPosition::DeletedIn(sequences_indices) => {
                            if sequences_indices.contains(&sequence_index) {
                                write!(f, "-")?
                            } else {
                                write!(
                                    f,
                                    "{}",
                                    self.sequences[sequence_index][current_sequence_position]
                                )?;
                                current_sequence_position += 1;
                            }
                        }
                        AlignedPosition::MatchedIn(_) => {
                            write!(
                                f,
                                "{}",
                                self.sequences[sequence_index][current_sequence_position]
                            )?;
                            current_sequence_position += 1;
                        }
                    }
                }
                writeln!(f, "")
            };

        let write_alignment_according_to_sequence =
            |sequence_index: usize, f: &mut std::fmt::Formatter<'_>| -> std::fmt::Result {
                for position in &self.alignment {
                    match position {
                        AlignedPosition::DeletedIn(_) => write!(f, " ")?,
                        AlignedPosition::MatchedIn(sequences_indices) => {
                            if sequences_indices.contains(&sequence_index)
                                && sequences_indices.contains(&(sequence_index - 1))
                            {
                                write!(f, "|")?;
                            } else {
                                write!(f, " ")?;
                            }
                        }
                    }
                }
                writeln!(f, "")
            };

        if !self.sequences.is_empty() {
            writeln!(f, "")?;
            write_sequence_according_to_alignment(0, f)?;

            for seqence_index in 1..self.sequences.len() {
                write_alignment_according_to_sequence(seqence_index, f)?;
                write_sequence_according_to_alignment(seqence_index, f)?;
            }
            Ok(())
        } else {
            writeln!(f, "Empty alignement")
        }
    }
}

pub mod global {
    pub mod needleman_wunsch {

        use std::{collections::HashMap, marker::PhantomData};

        use crate::{
            substitution_matrix::dna::SIMPLE as substitution_matrix, AlignedPosition, Alignment,
            Global,
        };
        use nalgebra::DMatrix;
        use sequence_language::{Sequence, DNA};

        pub fn align(
            sequence_a: Sequence<DNA>,
            sequence_b: Sequence<DNA>,
        ) -> Alignment<Global, DNA> {
            // we add one to accomodate for the gap symbol
            let size_a = sequence_a.len() + 1;
            let size_b = sequence_b.len() + 1;

            let gap_cost = -1;

            let mut alignment_matrix = DMatrix::<isize>::zeros(size_a, size_b);

            // key is position at (x1, y1), value is predecessor (x2, y2) according to maximum score
            let mut predecessor: HashMap<(usize, usize), (usize, usize)> = HashMap::new();

            // initilaize first row
            for (position, score) in alignment_matrix.row_mut(0).iter_mut().enumerate().skip(1) {
                *score = position as isize * gap_cost;
                predecessor.insert((0, position), (0, position - 1));
            }

            // initilaize first column
            for (position, score) in alignment_matrix
                .column_mut(0)
                .iter_mut()
                .enumerate()
                .skip(1)
            {
                *score = position as isize * gap_cost;
                predecessor.insert((position, 0), (position - 1, 0));
            }

            for i in 1..size_a {
                for j in 1..size_b {
                    let predecessor_candidates = [
                        // step right
                        (i - 1, j),
                        // step diagonal
                        (i - 1, j - 1),
                        // step down
                        (i, j - 1),
                    ];

                    let scores = [
                        // step right
                        alignment_matrix[predecessor_candidates[0]] + gap_cost,
                        // step diagonal
                        alignment_matrix[predecessor_candidates[1]]
                            + substitution_matrix[(sequence_a[i - 1], sequence_b[j - 1])],
                        // step down
                        alignment_matrix[predecessor_candidates[2]] + gap_cost,
                    ];

                    let (index, score) = scores
                        .iter()
                        .cloned()
                        .enumerate()
                        .max_by_key(|(_, score)| *score)
                        .expect("could not fing maximun, why? :(");

                    alignment_matrix[(i, j)] = score;

                    predecessor.insert((i, j), predecessor_candidates[index]);
                }
            }

            let mut current_position = (size_a - 1, size_b - 1);

            let mut path = vec![current_position];

            let mut alignment = Vec::new();

            while let Some(&predecessor_position) = predecessor.get(&current_position) {
                // deletion in sequence_a / insertion in sequence_b
                if current_position.0 == predecessor_position.0 {
                    alignment.push(AlignedPosition::DeletedIn(vec![0]));
                }
                // deletion in sequence_b / insertion in sequence_a
                else if current_position.1 == predecessor_position.1 {
                    alignment.push(AlignedPosition::DeletedIn(vec![1]));
                } else {
                    // match
                    if sequence_a[current_position.0 - 1] == sequence_b[current_position.1 - 1] {
                        alignment.push(AlignedPosition::MatchedIn(vec![0, 1]));
                    }
                    // mismatch
                    else {
                        alignment.push(AlignedPosition::MatchedIn(vec![]));
                    }
                }

                current_position = predecessor_position;
                path.push(current_position);
            }

            // we walked the path backwards, "backtracking", remember?
            alignment.reverse();

            let alignment: Alignment<Global, _> = Alignment {
                alignment_type: PhantomData,
                sequences: vec![sequence_a, sequence_b],
                alignment,
            };

            println!("Alignment: {}", alignment);

            alignment
        }
    }
}

mod substitution_matrix {
    pub mod dna {
        use std::ops::Index;

        use nalgebra::{matrix, Matrix4};
        use sequence_language::{
            dna::nucleobase::{A, C, G, T},
            DNA,
        };

        pub struct DnaSubstitutionMatrix(Matrix4<isize>);

        pub const SIMPLE: DnaSubstitutionMatrix = DnaSubstitutionMatrix(matrix![
        //  A   C   G   T
            1, -1, -1, -1; // A
            -1, 1, -1, -1; // C
            -1, -1, 1, -1; // G
            -1, -1, -1, 1; // T
        ]);

        impl Index<(DNA, DNA)> for DnaSubstitutionMatrix {
            type Output = isize;

            fn index(&self, index: (DNA, DNA)) -> &Self::Output {
                fn to_index(nucleobase: DNA) -> usize {
                    match nucleobase {
                        A => 0,
                        C => 1,
                        G => 2,
                        T => 3,
                    }
                }

                &self.0[(to_index(index.0), to_index(index.1))]
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use sequence_language::{Sequence, DNA};

    use super::global::needleman_wunsch;

    #[test]
    fn it_works() -> anyhow::Result<()> {
        let sequence_a: Sequence<DNA> = "AACGT".try_into()?;

        let sequence_b: Sequence<DNA> = "ACCGTT".try_into()?;

        needleman_wunsch::align(sequence_a, sequence_b);

        Ok(())
    }

    #[test]
    fn wikipedia_example_one() -> anyhow::Result<()> {
        let sequence_a: Sequence<DNA> = "GCATGCG".try_into()?;

        let sequence_b: Sequence<DNA> = "GATTACA".try_into()?;

        needleman_wunsch::align(sequence_a, sequence_b);

        Ok(())
    }

    #[test]
    fn wikipedia_example_two() -> anyhow::Result<()> {
        let sequence_a: Sequence<DNA> = "GAAAAAAT".try_into()?;

        let sequence_b: Sequence<DNA> = "GAAT".try_into()?;

        needleman_wunsch::align(sequence_a, sequence_b);

        Ok(())
    }

    // #[test]
    // fn protein_alignment() -> anyhow::Result<()> {
    //     let sequence_a: Sequence<Protein> =
    //         "MQIFVKTLTGKTITLEVEPSDTIENVKAKIQDKEGIPPDQQRLIFAGKQLEDGRTLSDYNIQKESTLHLVLRLRGG"
    //             .try_into()?;

    //     let sequence_b: Sequence<Protein> =
    //         "MADEKPKEGVKTENNDHINLKVAGQDGSVVQFKIKRHTPLSKLMKAYCERQGLSMRQIRFRFDGQPINETDTPAQLEMEDEDTIDVFQQQTGG".try_into()?;

    //     needleman_wunsch::align(sequence_a, sequence_b);

    //     Ok(())
    // }
}
