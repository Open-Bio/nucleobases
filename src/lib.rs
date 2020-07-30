
use crate::Nucleobase::{Adenine, Cytosine, Guanine, Thymine, Uracil};

#[derive(std::fmt::Debug)]
#[derive(std::cmp::PartialEq)]
pub enum Nucleobase {
    Adenine,
    Cytosine,
    Guanine,
    Thymine,
    Uracil,
}

impl Nucleobase {
    pub fn is_purine(&self) -> bool {
        match self {
            Adenine | Guanine => true,
            Cytosine | Thymine | Uracil => false,
        }
    }

    pub fn is_pyrimidine(&self) -> bool {
        match self {
            Adenine | Guanine => false,
            Cytosine | Thymine | Uracil => true,
        }
    }

    pub fn is_ketone(&self) -> bool {
        match self {
            Adenine | Cytosine => false,
            Guanine | Thymine | Uracil => true,
        }
    }

    pub fn is_amine(&self) -> bool {
        match self {
            Adenine | Cytosine => true,
            Guanine | Thymine | Uracil => false,
        }
    }

    pub fn is_ribonucleotide_base(&self) -> bool {
        match self {
            Adenine | Cytosine | Guanine | Uracil => true,
            Thymine => false,
        }
    }

    pub fn is_deoxyribonucleotide_base(&self) -> bool {
        match self {
            Adenine | Cytosine | Guanine | Thymine => true,
            Uracil => false
        }
    }

    pub fn letter_code(&self) -> char {
        match self {
            Adenine => 'A',
            Cytosine => 'C',
            Guanine => 'G',
            Thymine => 'T',
            Uracil => 'U',
        }
    }

    pub fn from_letter_code(letter_code: char) -> Option<Nucleobase> {
        match letter_code {
            'A' | 'a' => Some(Adenine),
            'C' | 'c' => Some(Cytosine),
            'G' | 'g' => Some(Guanine),
            'T' | 't' => Some(Thymine),
            'U' | 'u' => Some(Uracil),
            _ => None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Nucleobase;
    use crate::{Adenine, Cytosine, Guanine, Thymine, Uracil};

    #[test]
    fn correct_deoxyribonucleotide_bases() {
        assert_eq!(Adenine.is_deoxyribonucleotide_base(), true);
        assert_eq!(Cytosine.is_deoxyribonucleotide_base(), true);
        assert_eq!(Guanine.is_deoxyribonucleotide_base(), true);
        assert_eq!(Thymine.is_deoxyribonucleotide_base(), true);
        assert_eq!(Uracil.is_deoxyribonucleotide_base(), false);
    }

    #[test]
    fn correct_ribonucleotide_bases() {
        assert_eq!(Adenine.is_ribonucleotide_base(), true);
        assert_eq!(Cytosine.is_ribonucleotide_base(), true);
        assert_eq!(Guanine.is_ribonucleotide_base(), true);
        assert_eq!(Thymine.is_ribonucleotide_base(), false);
        assert_eq!(Uracil.is_ribonucleotide_base(), true);
    }

    #[test]
    fn correct_letter_codes() {
        assert_eq!(Nucleobase::from_letter_code('a'), Some(Adenine));
        assert_eq!(Nucleobase::from_letter_code('A'), Some(Adenine));
        assert_eq!(Nucleobase::from_letter_code('c'), Some(Cytosine));
        assert_eq!(Nucleobase::from_letter_code('C'), Some(Cytosine));
        assert_eq!(Nucleobase::from_letter_code('g'), Some(Guanine));
        assert_eq!(Nucleobase::from_letter_code('G'), Some(Guanine));
        assert_eq!(Nucleobase::from_letter_code('t'), Some(Thymine));
        assert_eq!(Nucleobase::from_letter_code('T'), Some(Thymine));
        assert_eq!(Nucleobase::from_letter_code('u'), Some(Uracil));
        assert_eq!(Nucleobase::from_letter_code('U'), Some(Uracil));
        assert_eq!(Nucleobase::from_letter_code('n'), None);
    }
}
