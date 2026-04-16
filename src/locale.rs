use icu_casemap::{CaseMapper, CaseMapperBorrowed};
use icu_locale::Locale as ICULocale;

pub struct Locale<'a> {
    mapper: CaseMapperBorrowed<'a>,
    icu: ICULocale,
}

impl<'a> Locale<'a> {
    pub fn try_new(descriptor: &str) -> Option<Self> {
        ICULocale::try_from_str(descriptor).ok().map(|icu| {
            let mapper = CaseMapper::new();
            Locale { mapper, icu }
        })
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_valid_descriptor() {
        super::Locale::try_new("en-us").unwrap();
    }

    #[test]
    fn test_invalid_descriptor() {
        assert!(super::Locale::try_new("").is_none());
    }
}
