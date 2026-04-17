use icu_casemap::{CaseMapper, CaseMapperBorrowed};
use icu_locale::Locale as ICULocale;
use icu_properties::props::{ChangesWhenLowercased, Lowercase};
use icu_properties::{CodePointSetData, CodePointSetDataBorrowed};
use writeable::Writeable;

use crate::case::Case;

pub struct Locale<'a> {
    mapper: CaseMapperBorrowed<'a>,
    lower: CodePointSetDataBorrowed<'a>,
    upper: CodePointSetDataBorrowed<'a>,
    icu: ICULocale,
}

impl<'a> Locale<'a> {
    pub fn try_new(descriptor: &str) -> Option<Self> {
        let icu = if descriptor.is_empty() {
            ICULocale::UNKNOWN
        } else {
            ICULocale::try_from_str(descriptor).ok()?
        };

        Some(Locale {
            mapper: CaseMapper::new(),
            lower: CodePointSetData::new::<Lowercase>(),
            upper: CodePointSetData::new::<ChangesWhenLowercased>(),
            icu,
        })
    }

    pub fn case(&self, character: char) -> Case {
        if self.lower.contains(character) {
            Case::Lower
        } else {
            if self.upper.contains(character) {
                Case::Upper
            } else {
                Case::Neutral
            }
        }
    }

    pub fn first_char_to_upper(&self, string: &'a str) -> impl Writeable + 'a {
        self.mapper
            .titlecase_segment_with_only_case_data(string, &self.icu.id, Default::default())
    }

    pub fn first_char_to_lower(&self, string: &'a str) -> impl Writeable + 'a {
        self.mapper.lowercase(string, &self.icu.id)
    }
}

#[cfg(test)]
mod tests {
    use super::Case;
    use writeable::Writeable;

    const UPPERCASE_DIGRAPH: char = '\u{01C4}';
    const TITLECASE_DIGRAPH: char = '\u{01C5}';
    const LOWERCASE_DIGRAPH: char = '\u{01C6}';

    fn locale<'a>(descriptor: &'a str) -> super::Locale<'a> {
        super::Locale::try_new(descriptor).unwrap()
    }

    #[test]
    fn test_valid_descriptor() {
        locale("en-US");
    }

    #[test]
    fn test_invalid_descriptor() {
        assert!(super::Locale::try_new("?").is_none());
    }

    #[test]
    fn test_upper() {
        assert_eq!(locale("").case('Є'), Case::Upper);
        assert_eq!(locale("").case(TITLECASE_DIGRAPH), Case::Upper);
        assert_eq!(locale("").case(UPPERCASE_DIGRAPH), Case::Upper);
    }

    #[test]
    fn test_lower() {
        assert_eq!(locale("").case('є'), Case::Lower);
        assert_eq!(locale("").case(LOWERCASE_DIGRAPH), Case::Lower);
    }

    #[test]
    fn test_neutral() {
        assert_eq!(locale("").case('1'), Case::Neutral);
        assert_eq!(locale("").case('-'), Case::Neutral);
        assert_eq!(locale("").case('«'), Case::Neutral);
        assert_eq!(locale("").case('\u{1f680}' /*rocket emoji*/), Case::Neutral);
    }

    #[test]
    fn test_to_upper() {
        assert_eq!(
            locale("")
                .first_char_to_upper("ii")
                .write_to_string()
                .into_owned(),
            "Ii"
        );
        assert_eq!(
            locale("")
                .first_char_to_upper("ß")
                .write_to_string()
                .into_owned(),
            "Ss"
        );
        assert_eq!(
            locale("tr-TR")
                .first_char_to_upper("ii")
                .write_to_string()
                .into_owned(),
            "İi"
        );
        assert_eq!(
            locale("")
                .first_char_to_upper(&LOWERCASE_DIGRAPH.to_string())
                .write_to_string()
                .into_owned(),
            TITLECASE_DIGRAPH.to_string(),
        );
    }

    #[test]
    fn test_to_lower() {
        assert_eq!(
            locale("")
                .first_char_to_lower("Ґрунт")
                .write_to_string()
                .into_owned(),
            "ґрунт"
        );
        assert_eq!(
            locale("")
                .first_char_to_lower(&TITLECASE_DIGRAPH.to_string())
                .write_to_string()
                .into_owned(),
            LOWERCASE_DIGRAPH.to_string(),
        );
        assert_eq!(
            locale("")
                .first_char_to_lower(&UPPERCASE_DIGRAPH.to_string())
                .write_to_string()
                .into_owned(),
            LOWERCASE_DIGRAPH.to_string(),
        );
    }
}
