use crate::{case::Case, locale::Locale};
use core::ops::FnOnce;

pub struct Row {
    string: String,
}

impl From<Row> for String {
    fn from(row: Row) -> Self {
        row.string
    }
}

impl From<String> for Row {
    fn from(mut string: String) -> Self {
        string.truncate(string.trim_end().len());
        Self { string }
    }
}

impl From<&str> for Row {
    fn from(string: &str) -> Self {
        let string: String = string.into();
        string.into()
    }
}

impl Row {
    pub fn case(&self, locale: &Locale) -> Option<Case> {
        self.string.chars().next().map(|c| locale.case(c))
    }

    pub fn first_char_to_upper(&mut self, locale: &Locale) {
        self.transform_first_char(|c| locale.to_title(c));
    }

    pub fn first_char_to_lower(&mut self, locale: &Locale) {
        self.transform_first_char(|c| locale.to_lower(c));
    }

    fn transform_first_char(&mut self, transform: impl FnOnce(&str) -> String) {
        let mut indices = self.string.char_indices();

        indices.next();

        if let Some((i, _)) = indices.next() {
            let mut new_string = transform(&self.string[..i]);

            new_string.push_str(&self.string[i..]);

            self.string = new_string;
        } else {
            self.string = transform(&self.string);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Locale, Row};

    #[test]
    fn test_trailing_whitespace() {
        let row: Row = "abc \t  \n".into();
        assert_eq!(row.string, "abc");
    }

    #[test]
    fn test_first_char_to_upper() {
        let locale: Locale = "".parse().unwrap();
        let string = "Some good Weather";
        let mut row: Row = string.into();

        assert_eq!(row.string, string);

        row.first_char_to_upper(&locale);

        assert_eq!(row.string, string);

        row = "some good Weather".into();

        row.first_char_to_upper(&locale);

        assert_eq!(row.string, string);
    }

    #[test]
    fn test_first_char_to_lower() {
        let locale: Locale = "".parse().unwrap();
        let string = "some good Weather";
        let mut row: Row = string.into();

        assert_eq!(row.string, string);

        row.first_char_to_lower(&locale);

        assert_eq!(row.string, string);

        let mut row: Row = string.into();

        row.first_char_to_lower(&locale);

        assert_eq!(row.string, "some good Weather");
    }
}
