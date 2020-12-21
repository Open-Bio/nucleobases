#![doc(html_root_url = "https://docs.rs/nucleobases/1.1.0")]

//! Standardized nucleobase data type
//! 
//! Type [`Nucleobase`](./enum.Nucleobase.html) represents a digital version of a physical nucleotide base in a
//! ribonucleic acid structure (DNA, RNA, mRNA, etc.).
//!
//! The [`Nucleobase`](./enum.Nucleobase.html) enum has 5 variants just as nucleobases that exist in nature. There's:
//! - [`Adenine`](./enum.Nucleobase.html#variant.Adenine)
//! - [`Cytosine`](./enum.Nucleobase.html#variant.Cytosine)
//! - [`Guanine`](./enum.Nucleobase.html#variant.Guanine)
//! - [`Thymine`](./enum.Nucleobase.html#variant.Thymine)
//! - [`Uracil`](./enum.Nucleobase.html#variant.Uracil)
//!
//! # No Compromises
//!
//! Astute eyes will notice the fact that [`Thymine`](./enum.Nucleobase.html#variant.Thymine) and
//! [`Uracil`](./enum.Nucleobase.html#variant.Uracil) have separate enumerations from one another
//! despite being nearly identical in every practical scenario. This is a conscious choice intended
//! to stand in-line with the Rust philosophy of having the least amount of compromises in code.
//! The goal is to allow users to determine how they handle [`Thymine`](./enum.Nucleobase.html#variant.Thymine)
//! and [`Uracil`](./enum.Nucleobase.html#variant.Uracil) without making asumptions about implementation
//! inside of the library itself.
//!
//! # Helper Methods
//!
//! For that reason, helper methods that determine whether or not a nucleobase is
//! [a deoxyribonucleotide base](./enum.Nucleobase.html#method.is_deoxyribonucleotide_base)
//! or [a ribonucleotide base](./enum.Nucleobase.html#method.is_ribonucleotide_base) have been included
//! for those who desire that functionality without limiting those who do not care.
//!
//! In addition to the methods indicating whether or not a base is a ribonucleotide base or
//! a deoxyribonucleotide base, there are helper methods for determining whether or not the
//! base is a:
//! - [Purine](./enum.Nucleobase.html#method.is_purine)
//! - [Pyrimidine](./enum.Nucleobase.html#method.is_pyrimidine)
//! - [Amine](./enum.Nucleobase.html#method.is_amine)
//! - [Ketone](./enum.Nucleobase.html#method.is_ketone)
//!
//! # Binary Serialization
//!
//! With the three sets of binary property methods provided the exact identity of a nucleobase
//! can be determined and serialized following the chart below.
//!
//! |            | Ketone           | Amine    |
//! |------------|------------------|----------|
//! | Purine     | Guanine          | Adenine  |
//! | Pyrimidine | Thymine / Uracil | Cytosine |
//! 
//! Whether a base is Thymine or Uracil comes down to whether it is a deoxyribonucleotide base or a
//! ribonucleotide base.
//!

use std::{
    marker::Copy,
    clone::Clone,
    hash::Hash,
    str::FromStr,
    convert::{
        TryFrom,
        Into,
        AsRef
    },
    fmt::{
        self,
        Debug,
        Display
    },
    cmp::{
        Eq,
        PartialEq,
        Ord,
        PartialOrd
    }
};

/// The [`Nucleobase`](./enum.Nucleobase.html) type. See [the module level documentation](../) for more.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Nucleobase {
    Adenine,
    Cytosine,
    Guanine,
    Thymine,
    Uracil,
}

impl Nucleobase {

    /// Returns whether a nucleobase is a purine or not.
    ///
    /// As a reminder, [`Adenine`](./enum.Nucleobase.html#variant.Adenine) and
    /// [`Guanine`](./enum.Nucleobase.html#variant.Guanine) are purines while
    /// [`Cytosine`](./enum.Nucleobase.html#variant.Cytosine),
    /// [`Thymine`](./enum.Nucleobase.html#variant.Thymine), and
    /// [`Uracil`](./enum.Nucleobase.html#variant.Uracil) are not.
    ///
    /// # Example
    ///
    /// This method can be helpful when parsing raw nucleobase data using iterators.
    ///
    /// ```should_panic
    /// # use nucleobases::Nucleobase;
    /// # use std::convert::TryFrom;
    /// #
    /// // C and T are not purines so this should fail
    /// assert!("GATTACA".chars()
    ///     .filter_map(|c| Nucleobase::try_from(c).ok())
    ///     .all(|base| base.is_purine()))
    /// ```
    pub fn is_purine(&self) -> bool {
        match self {
            Nucleobase::Adenine | Nucleobase::Guanine => true,
            Nucleobase::Cytosine | Nucleobase::Thymine | Nucleobase::Uracil => false,
        }
    }
    
    /// Returns whether a nucleobase is a pyrimidine or not.
    ///
    /// As a reminder, [`Cytosine`](./enum.Nucleobase.html#variant.Cytosine),
    /// [`Thymine`](./enum.Nucleobase.html#variant.Thymine), and
    /// [`Uracil`](./enum.Nucleobase.html#variant.Uracil) are pyrimidines while
    /// [`Adenine`](./enum.Nucleobase.html#variant.Adenine) and
    /// [`Guanine`](./enum.Nucleobase.html#variant.Guanine) are not.
    ///
    /// # Example
    ///
    /// This method can be helpful when parsing raw nucleobase data using iterators.
    ///
    /// ```should_panic
    /// # use nucleobases::Nucleobase;
    /// # use std::convert::TryFrom;
    /// #
    /// // A and G are not pyrimidines so this should fail
    /// assert!("GATTACA".chars()
    ///     .filter_map(|c| Nucleobase::try_from(c).ok())
    ///     .all(|base| base.is_pyrimidine()))
    /// ```
    pub fn is_pyrimidine(&self) -> bool {
        match self {
            Nucleobase::Adenine | Nucleobase::Guanine => false,
            Nucleobase::Cytosine | Nucleobase::Thymine | Nucleobase::Uracil => true,
        }
    }
    
    /// Returns whether a nucleobase is a purine or not.
    ///
    /// As a reminder, [`Adenine`](./enum.Nucleobase.html#variant.Adenine) and
    /// [`Cytosine`](./enum.Nucleobase.html#variant.Cytosine) are amines while
    /// [`Guanine`](./enum.Nucleobase.html#variant.Guanine),
    /// [`Thymine`](./enum.Nucleobase.html#variant.Thymine), and
    /// [`Uracil`](./enum.Nucleobase.html#variant.Uracil) are not.
    ///
    /// # Example
    ///
    /// This method can be helpful when parsing raw nucleobase data using iterators.
    ///
    /// ```should_panic
    /// # use nucleobases::Nucleobase;
    /// # use std::convert::TryFrom;
    /// #
    /// // G and T are not amines so this should fail
    /// assert!("GATTACA".chars()
    ///     .filter_map(|c| Nucleobase::try_from(c).ok())
    ///     .all(|base| base.is_amine()))
    /// ```
    pub fn is_amine(&self) -> bool {
        match self {
            Nucleobase::Adenine | Nucleobase::Cytosine => true,
            Nucleobase::Guanine | Nucleobase::Thymine | Nucleobase::Uracil => false,
        }
    }
    
    /// Returns whether a nucleobase is a ketone or not.
    ///
    /// As a reminder, [`Guanine`](./enum.Nucleobase.html#variant.Guanine),
    /// [`Thymine`](./enum.Nucleobase.html#variant.Thymine), and
    /// [`Uracil`](./enum.Nucleobase.html#variant.Uracil) are ketones while
    /// [`Adenine`](./enum.Nucleobase.html#variant.Adenine) and
    /// [`Cytosine`](./enum.Nucleobase.html#variant.Cytosine) are not.
    ///
    /// # Example
    ///
    /// This method can be helpful when parsing raw nucleobase data using iterators.
    ///
    /// ```should_panic
    /// # use nucleobases::Nucleobase;
    /// # use std::convert::TryFrom;
    /// #
    /// // A and C are not ketones so this should fail
    /// assert!("GATTACA".chars()
    ///     .filter_map(|c| Nucleobase::try_from(c).ok())
    ///     .all(|base| base.is_ketone()))
    /// ```
    pub fn is_ketone(&self) -> bool {
        match self {
            Nucleobase::Adenine | Nucleobase::Cytosine => false,
            Nucleobase::Guanine | Nucleobase::Thymine | Nucleobase::Uracil => true,
        }
    }
    
    /// Returns whether or not a nucleobase should exist in ribonucleic acid (RNA).
    ///
    /// As a reminder, [`Adenine`](./enum.Nucleobase.html#variant.Adenine),
    /// [`Cytosine`](./enum.Nucleobase.html#variant.Cytosine),
    /// [`Guanine`](./enum.Nucleobase.html#variant.Guanine), and
    /// [`Uracil`](./enum.Nucleobase.html#variant.Uracil) can exist in ribonucleic acid (RNA) while
    /// [`Thymine`](./enum.Nucleobase.html#variant.Thymine) should not be found in ribonucleic acid;
    /// all instances of it should be replaced with [`Uracil`](./enum.Nucleobase.html#variant.Uracil).
    /// 
    /// # Example
    ///
    /// This method can be helpful when parsing raw nucleobase data using iterators.
    ///
    /// ```should_panic
    /// # use nucleobases::Nucleobase;
    /// # use std::convert::TryFrom;
    /// #
    /// // T is not a ribonucleotide base so this should fail
    /// assert!("GATTACA".chars()
    ///     .filter_map(|c| Nucleobase::try_from(c).ok())
    ///     .all(|base| base.is_ribonucleotide_base()))
    /// ```
    pub fn is_ribonucleotide_base(&self) -> bool {
        match self {
            Nucleobase::Adenine | Nucleobase::Cytosine | Nucleobase::Guanine | Nucleobase::Uracil => true,
            Nucleobase::Thymine => false,
        }
    }

    /// Alias for [`is_ribonucleotide_base`](./enum.Nucleobase.html#method.is_ribonucleotide_base).
    pub fn is_rna_base(&self) -> bool {
        self.is_ribonucleotide_base()
    }

    /// Returns whether or not a nucleobase should exist in deoxyribonucleic acid (DNA).
    ///
    /// As a reminder, [`Adenine`](./enum.Nucleobase.html#variant.Adenine),
    /// [`Cytosine`](./enum.Nucleobase.html#variant.Cytosine),
    /// [`Guanine`](./enum.Nucleobase.html#variant.Guanine), and
    /// [`Thymine`](./enum.Nucleobase.html#variant.Thymine) can exist in deoxyribonucleic acid (DNA) while
    /// [`Uracil`](./enum.Nucleobase.html#variant.Uracicl) should not be found in ribonucleic acid;
    /// all instances of it should be replaced with [`Thymine`](./enum.Nucleobase.html#variant.Thymine).
    /// 
    /// # Example
    ///
    /// This method can be helpful when parsing raw nucleobase data using iterators.
    ///
    /// ```
    /// # use nucleobases::Nucleobase;
    /// # use std::convert::TryFrom;
    /// #
    /// // U is not a present so this should pass
    /// assert!("GATTACA".chars()
    ///     .filter_map(|c| Nucleobase::try_from(c).ok())
    ///     .all(|base| base.is_deoxyribonucleotide_base()))
    /// ```
    pub fn is_deoxyribonucleotide_base(&self) -> bool {
        match self {
            Nucleobase::Adenine | Nucleobase::Cytosine | Nucleobase::Guanine | Nucleobase::Thymine => true,
            Nucleobase::Uracil => false
        }
    }

    /// Alias for [`is_deoxyribonucleotide_base`](./enum.Nucleobase.html#method.is_deoxyribonucleotide_base).
    pub fn is_dna_base(&self) -> bool {
        self.is_deoxyribonucleotide_base()
    }

    /// Returns whether or not two nucleobases are the complements of one another.
    ///
    /// Two nucleobases are complements of one another if one
    /// [is a purine](./enum.Nucleobase.html#method.is_purine) and the other
    /// [is a pyrimidine](./enum.Nucleobase.html#method.is_pyrimidine) and if one [is
    /// an amine](./enum.Nucleobase.html#method.is_amine) and the other
    /// [is a ketone](./enum.Nucleobase.html#method.is_ketone).
    pub fn are_complementary(one: &Self, two: &Self) -> bool {
        // Because of the unique chemical structure of the nucleotides
        // they are only complements if one is a purine and the other
        // is a pyrimidine and one is an amine and the other is a ketone
        (one.is_purine() != two.is_purine()) && (one.is_amine() != two.is_amine())
    }

    /// Returns whether or not the given nucleobase is a complement or not.
    ///
    /// See [`are_complementary`](./enum.Nucleobase.html#method.are_complementary).
    pub fn is_complementary_to(&self, other: &Self) -> bool {
        Self::are_complementary(self, other)
    }

    /// Converts a nucleobase into its complementary nucleobase inside of deoxyribonucleic acid (DNA).
    ///
    /// For use in ribonucleic acid (RNA) see [`to_ribonucleotide_complement`](./enum.Nucleobase.html#method.to_ribonucleotide_complement).
    pub fn to_deoxyribonucleotide_complement(self) -> Nucleobase {
        match self {
            Nucleobase::Adenine => Nucleobase::Thymine,
            Nucleobase::Cytosine => Nucleobase::Guanine,
            Nucleobase::Guanine => Nucleobase::Cytosine,
            Nucleobase::Thymine => Nucleobase::Adenine,
            Nucleobase::Uracil => Nucleobase::Adenine
        }
    }

    /// Alias for [`to_deoxyribonucleotide_complement`](./enum.Nucleobase.html#method.to_deoxyribonucleotide_complement).
    pub fn to_dna_complement(self) -> Nucleobase {
        self.to_deoxyribonucleotide_complement()
    }

    /// Converts a nucleobase into its complementary nucleobase inside of ribonucleic acid (RNA).
    ///
    /// For use in deoxyribonucleic acid (DNA) see [`to_deoxyribonucleotide_complement`](./enum.Nucleobase.html#method.to_deoxyribonucleotide_complement).
    pub fn to_ribonucleotide_complement(self) -> Nucleobase {
        match self {
            Nucleobase::Adenine => Nucleobase::Thymine,
            Nucleobase::Cytosine => Nucleobase::Guanine,
            Nucleobase::Guanine => Nucleobase::Cytosine,
            Nucleobase::Thymine => Nucleobase::Adenine,
            Nucleobase::Uracil => Nucleobase::Adenine
        }
    }

    /// Alias for [`to_ribonucleotide_complement`](./enum.Nucleobase.html#method.to_ribonucleotide_complement).
    pub fn to_rna_complement(self) -> Nucleobase {
        self.to_ribonucleotide_complement()
    }
}

// Implemented for general coverage of conversions between characters and nucleobases 
impl TryFrom<char> for Nucleobase {
    type Error = ConversionError;

    /// ```
    /// # use nucleobases::Nucleobase;
    /// # use nucleobases::ConversionError::{*};
    /// # use std::convert::TryFrom;
    /// assert_eq!(Nucleobase::try_from('A'), Ok(Nucleobase::Adenine));
    /// assert_eq!(Nucleobase::try_from('a'), Ok(Nucleobase::Adenine));
    /// assert_eq!(Nucleobase::try_from('C'), Ok(Nucleobase::Cytosine));
    /// assert_eq!(Nucleobase::try_from('c'), Ok(Nucleobase::Cytosine));
    /// assert_eq!(Nucleobase::try_from('G'), Ok(Nucleobase::Guanine));
    /// assert_eq!(Nucleobase::try_from('g'), Ok(Nucleobase::Guanine));
    /// assert_eq!(Nucleobase::try_from('T'), Ok(Nucleobase::Thymine));
    /// assert_eq!(Nucleobase::try_from('t'), Ok(Nucleobase::Thymine));
    /// assert_eq!(Nucleobase::try_from('u'), Ok(Nucleobase::Uracil));
    /// assert_eq!(Nucleobase::try_from('U'), Ok(Nucleobase::Uracil));
    ///
    /// assert_eq!(Nucleobase::try_from('⻇'), Err(InvalidCharacterError('⻇')));
    /// ```
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'A' | 'a' => Ok(Nucleobase::Adenine),
            'C' | 'c' => Ok(Nucleobase::Cytosine),
            'G' | 'g' => Ok(Nucleobase::Guanine),
            'T' | 't' => Ok(Nucleobase::Thymine),
            'U' | 'u' => Ok(Nucleobase::Uracil),
            _ => Err(ConversionError::InvalidCharacterError(c))
        }
    }
}

impl FromStr for Nucleobase {
    type Err = ConversionError;

    /// ```
    /// # use nucleobases::Nucleobase;
    /// # use nucleobases::ConversionError::{*};
    /// # use std::str::FromStr;
    /// assert_eq!(Nucleobase::from_str("A"), Ok(Nucleobase::Adenine));
    /// assert_eq!(Nucleobase::from_str("a"), Ok(Nucleobase::Adenine));
    /// assert_eq!(Nucleobase::from_str("C"), Ok(Nucleobase::Cytosine));
    /// assert_eq!(Nucleobase::from_str("c"), Ok(Nucleobase::Cytosine));
    /// assert_eq!(Nucleobase::from_str("G"), Ok(Nucleobase::Guanine));
    /// assert_eq!(Nucleobase::from_str("g"), Ok(Nucleobase::Guanine));
    /// assert_eq!(Nucleobase::from_str("T"), Ok(Nucleobase::Thymine));
    /// assert_eq!(Nucleobase::from_str("t"), Ok(Nucleobase::Thymine));
    /// assert_eq!(Nucleobase::from_str("u"), Ok(Nucleobase::Uracil));
    /// assert_eq!(Nucleobase::from_str("U"), Ok(Nucleobase::Uracil));
    ///
    /// assert_eq!(Nucleobase::from_str("⻇"), Err(InvalidCharacterError('⻇')));
    /// assert_eq!(Nucleobase::from_str("GATTACA"), Err(StringLengthError));
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() == 1 {
            match &s[0..1] {
                "A" | "a" => Ok(Nucleobase::Adenine),
                "C" | "c" => Ok(Nucleobase::Cytosine),
                "G" | "g" => Ok(Nucleobase::Guanine),
                "T" | "t" => Ok(Nucleobase::Thymine),
                "U" | "u" => Ok(Nucleobase::Uracil),
                c => Err(ConversionError::InvalidCharacterError(c.chars().next().unwrap()))
            }
        } else {
            if s.chars().count() == 1 {
                Err(ConversionError::InvalidCharacterError(s.chars().next().unwrap()))
            } else {
                Err(ConversionError::StringLengthError)
            }
        }
    }
}

// Implemented specifically because conversions from Nucleobase to char can never fail.
// Everything I've read seems to imply you should never implement this trait because
// it's auto implemented, but it seems like this might be the only edge case in which
// it might be justifiable.
impl Into<char> for Nucleobase {
    /// ```
    /// # use nucleobases::Nucleobase::{*};
    /// # use nucleobases::ConversionError::{*};
    /// # use std::convert::From;
    /// assert!('A' == Adenine.into());
    /// assert!('C' == Cytosine.into());
    /// assert!('G' == Guanine.into());
    /// assert!('T' == Thymine.into());
    /// assert!('U' == Uracil.into());
    /// ```
    fn into(self) -> char {
        *self.as_ref()
    }
}

impl AsRef<char> for Nucleobase {
    fn as_ref(&self) -> &char {
        match self {
            Nucleobase::Adenine => &'A',
            Nucleobase::Cytosine => &'C',
            Nucleobase::Guanine => &'G',
            Nucleobase::Thymine => &'T',
            Nucleobase::Uracil => &'U'
        }
    }
}

impl AsRef<str> for Nucleobase {
    fn as_ref(&self) -> &str {
        match self {
            Nucleobase::Adenine => &"A",
            Nucleobase::Cytosine => &"C",
            Nucleobase::Guanine => &"G",
            Nucleobase::Thymine => &"T",
            Nucleobase::Uracil => &"U"
        }
    }
}

// As a result of this implementation Nucleobase also auto implements
// ToString and does so by basically just coercing Nucleobase into
// a &str and then converting it into a String.
impl Display for Nucleobase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Self can be passed directly through type coercion as
        // Nucleobase implements AsRef<str>
        write!(f, "{}", self)
    }
}

/// The error type for conversions between [`Nucleobase`](./enum.Nucleobase.html) and other types
#[derive(Debug, PartialEq)]
pub enum ConversionError {
    InvalidCharacterError(char),
    StringLengthError
}

impl Display for ConversionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use ConversionError::{*};

        match self {
            InvalidCharacterError(c) => write!(f, "invalid character: cannot convert {} into a nucleobase", c),
            StringLengthError => write!(f, "cannot convert string of length greater than 1 character into a single nucleobase")
        }
    }
}
