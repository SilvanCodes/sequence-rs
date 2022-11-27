pub mod global {
    pub mod needleman_wunsch {

        use std::collections::HashMap;

        use crate::substitution_matrix::dna::SIMPLE as substitution_matrix;
        use nalgebra::DMatrix;
        use sequence_language::{Sequence, DNA};

        pub fn align(sequence_a: Sequence<DNA>, sequence_b: Sequence<DNA>) {
            // we add one to accomodate for the gap symbol
            let size_a = sequence_a.len() + 1;
            let size_b = sequence_b.len() + 1;

            let gap_cost = -2;

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
                    dbg!(i, j);

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

            dbg!(&alignment_matrix);

            // dbg!(&predecessor);

            // TODO:
            // - compute path from predecessors
            // - compute alignment from path
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
        let sequence_a: Sequence<DNA> = "ACTG".try_into()?;

        let sequence_b: Sequence<DNA> = "ACTG".try_into()?;

        needleman_wunsch::align(sequence_a, sequence_b);

        Ok(())
    }
}
