use std::marker::Copy;
use std::clone::Clone;
use std::hash::Hash;

use std::fmt;
use std::cmp;

#[derive(Copy, Clone, cmp::Eq, cmp::PartialEq, cmp::Ord, cmp::PartialOrd, Hash, fmt::Debug)]
pub enum Nucleobase {
    Adenine,
    Cytosine,
    Guanine,
    Thymine,
    Uracil,
}

pub fn is_purine(base: &Nucleobase) -> bool {
    match base {
        Nucleobase::Adenine | Nucleobase::Guanine => true,
        Nucleobase::Cytosine | Nucleobase::Thymine | Nucleobase::Uracil => false,
    }
}

pub fn is_pyrimidine(base: &Nucleobase) -> bool {
    match base {
        Nucleobase::Adenine | Nucleobase::Guanine => false,
        Nucleobase::Cytosine | Nucleobase::Thymine | Nucleobase::Uracil => true,
    }
}

pub fn is_ketone(base: &Nucleobase) -> bool {
    match base {
        Nucleobase::Adenine | Nucleobase::Cytosine => false,
        Nucleobase::Guanine | Nucleobase::Thymine | Nucleobase::Uracil => true,
    }
}

pub fn is_amine(base: &Nucleobase) -> bool {
    match base {
        Nucleobase::Adenine | Nucleobase::Cytosine => true,
        Nucleobase::Guanine | Nucleobase::Thymine | Nucleobase::Uracil => false,
    }
}

pub fn is_ribonucleotide_base(base: &Nucleobase) -> bool {
    match base {
        Nucleobase::Adenine | Nucleobase::Cytosine | Nucleobase::Guanine | Nucleobase::Uracil => true,
        Nucleobase::Thymine => false,
    }
}

pub fn is_deoxyribonucleotide_base(base: &Nucleobase) -> bool {
    match base {
        Nucleobase::Adenine | Nucleobase::Cytosine | Nucleobase::Guanine | Nucleobase::Thymine => true,
        Nucleobase::Uracil => false
    }
}

pub fn to_letter_code(base: &Nucleobase) -> char {
    match base {
        Nucleobase::Adenine => 'A',
        Nucleobase::Cytosine => 'C',
        Nucleobase::Guanine => 'G',
        Nucleobase::Thymine => 'T',
        Nucleobase::Uracil => 'U',
    }
}

pub fn from_letter_code(letter_code: &char) -> Option<Nucleobase> {
    match letter_code {
        'A' | 'a' => Some(Nucleobase::Adenine),
        'C' | 'c' => Some(Nucleobase::Cytosine),
        'G' | 'g' => Some(Nucleobase::Guanine),
        'T' | 't' => Some(Nucleobase::Thymine),
        'U' | 'u' => Some(Nucleobase::Uracil),
        _ => None
    }
}

impl Nucleobase {
    pub fn is_purine(&self) -> bool { is_purine(self) }

    pub fn is_pyrimidine(&self) -> bool { is_pyrimidine(self) }

    pub fn is_ketone(&self) -> bool { is_ketone(self) }

    pub fn is_amine(&self) -> bool { is_amine(self) }

    pub fn is_ribonucleotide_base(&self) -> bool { is_ribonucleotide_base(self) }

    pub fn is_deoxyribonucleotide_base(&self) -> bool { is_deoxyribonucleotide_base(self) }

    pub fn to_letter_code(&self) -> char { to_letter_code(self) }

    pub fn from_letter_code(letter_code: &char) -> Option<Self> { from_letter_code(letter_code) }
}

impl fmt::Display for Nucleobase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[cfg(test)]
mod tests {
    use crate::Nucleobase;

    #[test]
    fn correct_deoxyribonucleotide_bases() {
        assert_eq!(Nucleobase::Adenine.is_deoxyribonucleotide_base(), true);
        assert_eq!(Nucleobase::Cytosine.is_deoxyribonucleotide_base(), true);
        assert_eq!(Nucleobase::Guanine.is_deoxyribonucleotide_base(), true);
        assert_eq!(Nucleobase::Thymine.is_deoxyribonucleotide_base(), true);
        assert_eq!(Nucleobase::Uracil.is_deoxyribonucleotide_base(), false);
    }

    #[test]
    fn correct_ribonucleotide_bases() {
        assert_eq!(Nucleobase::Adenine.is_ribonucleotide_base(), true);
        assert_eq!(Nucleobase::Cytosine.is_ribonucleotide_base(), true);
        assert_eq!(Nucleobase::Guanine.is_ribonucleotide_base(), true);
        assert_eq!(Nucleobase::Thymine.is_ribonucleotide_base(), false);
        assert_eq!(Nucleobase::Uracil.is_ribonucleotide_base(), true);
    }

    #[test]
    fn correct_letter_codes() {
        assert_eq!(crate::from_letter_code(& 'a'), Some(Nucleobase::Adenine));
        assert_eq!(crate::from_letter_code(& 'A'), Some(Nucleobase::Adenine));
        assert_eq!(crate::from_letter_code(& 'c'), Some(Nucleobase::Cytosine));
        assert_eq!(crate::from_letter_code(& 'C'), Some(Nucleobase::Cytosine));
        assert_eq!(crate::from_letter_code(& 'g'), Some(Nucleobase::Guanine));
        assert_eq!(crate::from_letter_code(& 'G'), Some(Nucleobase::Guanine));
        assert_eq!(crate::from_letter_code(& 't'), Some(Nucleobase::Thymine));
        assert_eq!(crate::from_letter_code(& 'T'), Some(Nucleobase::Thymine));
        assert_eq!(crate::from_letter_code(& 'u'), Some(Nucleobase::Uracil));
        assert_eq!(crate::from_letter_code(& 'U'), Some(Nucleobase::Uracil));
        assert_eq!(crate::from_letter_code(& 'n'), None);
    }
}
