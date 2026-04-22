use crate::case::Case;
use crate::locale::Locale;

pub struct Row {
    string: String,
}

impl Row {
    pub fn new(mut string: String) -> Row {
        string.truncate(string.trim_end().len());
        Row { string }
    }

    pub fn case(&self, locale: &Locale) -> Option<Case> {
        match self.string.chars().next() {
            Some(character) => Some(locale.case(character)),
            _ => None,
        }
    }

    pub fn first_char_to_upper(&mut self, locale: &Locale) {
        self.string = locale.first_char_to_lower(&self.string);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_trailing_whitespace() {
        let row = super::Row::new(String::from("abc \t  \n"));
        assert_eq!(row.string, "abc");
    }
}
