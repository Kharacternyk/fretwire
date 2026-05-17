use core::ops::FnOnce;

use crate::case::Case;
use crate::locale::Locale;

pub struct Row {
    string: String,
}

impl From<Row> for String {
    fn from(row: Row) -> String {
        row.string
    }
}

impl Row {
    pub fn new(mut string: String) -> Row {
        string.truncate(string.trim_end().len());
        Row { string }
    }

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
    use crate::locale::Locale;

    #[test]
    fn test_trailing_whitespace() {
        let row = super::Row::new(String::from("abc \t  \n"));
        assert_eq!(row.string, "abc");
    }

    #[test]
    fn test_first_char_to_upper() {
        let locale = Locale::try_new("").unwrap();
        let string = "Some good Weather";
        let mut row = super::Row::new(String::from(string));

        assert_eq!(row.string, string);

        row.first_char_to_upper(&locale);

        assert_eq!(row.string, string);

        row = super::Row::new(String::from("some good Weather"));

        row.first_char_to_upper(&locale);

        assert_eq!(row.string, string);
    }

    #[test]
    fn test_first_char_to_lower() {
        let locale = Locale::try_new("").unwrap();
        let string = "some good Weather";
        let mut row = super::Row::new(String::from(string));

        assert_eq!(row.string, string);

        row.first_char_to_lower(&locale);

        assert_eq!(row.string, string);

        row = super::Row::new(String::from("Some good Weather"));

        row.first_char_to_lower(&locale);

        assert_eq!(row.string, "some good Weather");
    }
}
