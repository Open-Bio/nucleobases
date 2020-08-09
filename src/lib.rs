
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

pub fn is_purine(base: &Nucleobase) -> bool {
    match base {
        Adenine | Guanine => true,
        Cytosine | Thymine | Uracil => false,
    }
}

pub fn is_pyrimidine(base: &Nucleobase) -> bool {
    match base {
        Adenine | Guanine => false,
        Cytosine | Thymine | Uracil => true,
    }
}

pub fn is_ketone(base: &Nucleobase) -> bool {
    match base {
        Adenine | Cytosine => false,
        Guanine | Thymine | Uracil => true,
    }
}

pub fn is_amine(base: &Nucleobase) -> bool {
    match base {
        Adenine | Cytosine => true,
        Guanine | Thymine | Uracil => false,
    }
}

pub fn is_ribonucleotide_base(base: &Nucleobase) -> bool {
    match base {
        Adenine | Cytosine | Guanine | Uracil => true,
        Thymine => false,
    }
}

pub fn is_deoxyribonucleotide_base(base: &Nucleobase) -> bool {
    match base {
        Adenine | Cytosine | Guanine | Thymine => true,
        Uracil => false
    }
}

pub fn letter_code(base: &Nucleobase) -> char {
    match base {
        Adenine => 'A',
        Cytosine => 'C',
        Guanine => 'G',
        Thymine => 'T',
        Uracil => 'U',
    }
}

pub fn from_letter_code(letter_code: &char) -> Option<Nucleobase> {
    match letter_code {
        'A' | 'a' => Some(Adenine),
        'C' | 'c' => Some(Cytosine),
        'G' | 'g' => Some(Guanine),
        'T' | 't' => Some(Thymine),
        'U' | 'u' => Some(Uracil),
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

    pub fn letter_code(&self) -> char { letter_code(self) }

    pub fn from_letter_code(letter_code: &char) -> Option<Nucleobase> { from_letter_code(letter_code) }
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
        assert_eq!(Nucleobase::from_letter_code(& 'a'), Some(Adenine));
        assert_eq!(Nucleobase::from_letter_code(& 'A'), Some(Adenine));
        assert_eq!(Nucleobase::from_letter_code(& 'c'), Some(Cytosine));
        assert_eq!(Nucleobase::from_letter_code(& 'C'), Some(Cytosine));
        assert_eq!(Nucleobase::from_letter_code(& 'g'), Some(Guanine));
        assert_eq!(Nucleobase::from_letter_code(& 'G'), Some(Guanine));
        assert_eq!(Nucleobase::from_letter_code(& 't'), Some(Thymine));
        assert_eq!(Nucleobase::from_letter_code(& 'T'), Some(Thymine));
        assert_eq!(Nucleobase::from_letter_code(& 'u'), Some(Uracil));
        assert_eq!(Nucleobase::from_letter_code(& 'U'), Some(Uracil));
        assert_eq!(Nucleobase::from_letter_code(& 'n'), None);
    }
}
