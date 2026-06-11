use crate::{
    Case::{self, Lower, Upper},
    CaseRelation::{self, Stable, Unstable},
};
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
    pub fn case_relation(&self, character: char) -> CaseRelation {
        if self.lower.contains(character) {
            Unstable(Lower)
        } else if self.upper.contains(character) {
            Unstable(Upper)
        } else {
            Stable
        }
    }

    pub fn change_first_char_case(&self, string: &mut String, case: Case) {
        let i = string
            .char_indices()
            .nth(1)
            .map_or(string.len(), |(i, _)| i);

        let transformed = match case {
            Upper => self
                .mapper
                .titlecase_segment_with_only_case_data(
                    &string[..i],
                    &self.icu.id,
                    Default::default(),
                )
                .write_to_string()
                .into_owned(),
            Lower => self
                .mapper
                .lowercase(&string[..i], &self.icu.id)
                .write_to_string()
                .into_owned(),
        };

        string.replace_range(..i, &transformed);
    }

    #[must_use]
    pub fn compare(&self, a: &str, b: &str) -> Ordering {
        self.collator.compare(a, b)
    }
}

#[cfg(test)]
mod tests {
    use super::{Locale, Lower, Stable, Unstable, Upper};

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
        let locale = locale("");
        for character in ['Є', TITLECASE_DIGRAPH, UPPERCASE_DIGRAPH] {
            assert_eq!(locale.case_relation(character), Unstable(Upper));
        }
    }

    #[test]
    fn test_lower() {
        let locale = locale("");
        for character in ['є', LOWERCASE_DIGRAPH] {
            assert_eq!(locale.case_relation(character), Unstable(Lower));
        }
    }

    #[test]
    fn test_neutral() {
        let locale = locale("");
        for character in ['1', '-', '«', '\u{1f680}' /*rocket emoji*/] {
            assert_eq!(locale.case_relation(character), Stable);
        }
    }

    #[test]
    fn test_to_upper() {
        let titlecase_digraph = TITLECASE_DIGRAPH.to_string();

        for (descriptor, mut input, output) in [
            ("", "ii jj".into(), "Ii jj"),
            ("", "ßS".into(), "SsS"),
            ("tr-TR", "ii".into(), "İi"),
            ("", LOWERCASE_DIGRAPH.to_string(), &titlecase_digraph),
        ] {
            locale(descriptor).change_first_char_case(&mut input, Upper);
            assert_eq!(input, output);
        }
    }

    #[test]
    fn test_to_lower() {
        let locale = locale("");
        let lowercase_digraph = LOWERCASE_DIGRAPH.to_string();

        for (mut input, output) in [
            ("Ґрунт: А".into(), "ґрунт: А"),
            (TITLECASE_DIGRAPH.to_string(), &lowercase_digraph),
            (UPPERCASE_DIGRAPH.to_string(), &lowercase_digraph),
        ] {
            locale.change_first_char_case(&mut input, Lower);
            assert_eq!(input, output);
        }
    }
}
