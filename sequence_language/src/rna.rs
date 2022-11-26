pub mod nucleobase {
    pub use Nucleobase::Adenine;
    pub use Nucleobase::Cytosine;
    pub use Nucleobase::Guanine;
    pub use Nucleobase::Uracil;

    pub use Nucleobase::Adenine as A;
    pub use Nucleobase::Cytosine as C;
    pub use Nucleobase::Guanine as G;
    pub use Nucleobase::Uracil as U;

    pub enum Nucleobase {
        Adenine,
        Cytosine,
        Guanine,
        Uracil,
    }
}
