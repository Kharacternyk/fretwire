use crate::Case;
use core::cmp::Ordering;
use icu_casemap::{CaseMapper, CaseMapperBorrowed};
use icu_collator::{Collator, CollatorBorrowed};
use icu_locale::Locale as ICULocale;
use icu_properties::{
    CodePointSetData, CodePointSetDataBorrowed,
    props::{ChangesWhenLowercased, Lowercase},
};
use std::{str::FromStr, sync::Arc};
use writeable::Writeable;

#[derive(Clone)]
pub struct Locale {
    mapper: CaseMapperBorrowed<'static>,
    lower: CodePointSetDataBorrowed<'static>,
    upper: CodePointSetDataBorrowed<'static>,
    collator: Arc<CollatorBorrowed<'static>>,
    icu: ICULocale,
}

impl FromStr for Locale {
    type Err = ();

    fn from_str(descriptor: &str) -> Result<Self, ()> {
        let icu = if descriptor.is_empty() {
            ICULocale::UNKNOWN
        } else {
            ICULocale::try_from_str(descriptor).map_err(|_| ())?
        };

        let preferences = icu.clone().into();
        let options = Default::default();
        let collator = Arc::new(Collator::try_new(preferences, options).map_err(|_| ())?);

        Ok(Self {
            mapper: CaseMapper::new(),
            lower: CodePointSetData::new::<Lowercase>(),
            upper: CodePointSetData::new::<ChangesWhenLowercased>(),
            collator,
            icu,
        })
    }
}

impl Locale {
    #[must_use] 
    pub fn case(&self, character: char) -> Case {
        if self.lower.contains(character) {
            Case::Lower
        } else if self.upper.contains(character) {
            Case::Upper
        } else {
            Case::Neutral
        }
    }

    #[must_use] 
    pub fn to_title(&self, string: &str) -> String {
        self.mapper
            .titlecase_segment_with_only_case_data(string, &self.icu.id, Default::default())
            .write_to_string()
            .into_owned()
    }

    #[must_use] 
    pub fn to_lower(&self, string: &str) -> String {
        self.mapper
            .lowercase(string, &self.icu.id)
            .write_to_string()
            .into_owned()
    }

    #[must_use] 
    pub fn compare(&self, a: &str, b: &str) -> Ordering {
        self.collator.compare(a, b)
    }
}

#[cfg(test)]
mod tests {
    use super::{
        Case::{Lower, Neutral, Upper},
        Locale,
    };

    const UPPERCASE_DIGRAPH: char = '\u{01C4}';
    const TITLECASE_DIGRAPH: char = '\u{01C5}';
    const LOWERCASE_DIGRAPH: char = '\u{01C6}';

    fn locale(descriptor: &str) -> Locale {
        descriptor.parse().unwrap()
    }

    #[test]
    fn test_valid_descriptors() {
        locale("uk-UA");
        locale("en-US");
        locale("en-GB");
    }

    #[test]
    fn test_invalid_descriptor() {
        assert!("?".parse::<Locale>().is_err());
    }

    #[test]
    fn test_upper() {
        assert_eq!(locale("").case('Є'), Upper);
        assert_eq!(locale("").case(TITLECASE_DIGRAPH), Upper);
        assert_eq!(locale("").case(UPPERCASE_DIGRAPH), Upper);
    }

    #[test]
    fn test_lower() {
        assert_eq!(locale("").case('є'), Lower);
        assert_eq!(locale("").case(LOWERCASE_DIGRAPH), Lower);
    }

    #[test]
    fn test_neutral() {
        assert_eq!(locale("").case('1'), Neutral);
        assert_eq!(locale("").case('-'), Neutral);
        assert_eq!(locale("").case('«'), Neutral);
        assert_eq!(locale("").case('\u{1f680}' /*rocket emoji*/), Neutral);
    }

    #[test]
    fn test_to_upper() {
        assert_eq!(locale("").to_title("ii"), "Ii");
        assert_eq!(locale("").to_title("ßS"), "Sss");
        assert_eq!(locale("tr-TR").to_title("ii"), "İi");
        assert_eq!(
            locale("").to_title(&LOWERCASE_DIGRAPH.to_string()),
            TITLECASE_DIGRAPH.to_string(),
        );
    }

    #[test]
    fn test_to_lower() {
        assert_eq!(locale("").to_lower("Ґрунт: А"), "ґрунт: а");
        assert_eq!(
            locale("").to_lower(&TITLECASE_DIGRAPH.to_string()),
            LOWERCASE_DIGRAPH.to_string(),
        );
        assert_eq!(
            locale("").to_lower(&UPPERCASE_DIGRAPH.to_string()),
            LOWERCASE_DIGRAPH.to_string(),
        );
    }
}
