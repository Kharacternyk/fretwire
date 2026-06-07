use super::Locale;
use crate::case::Case;

pub trait CasedString {
    fn first_char_case(&self, locale: &Locale) -> Option<Case>;
    fn first_char_to_upper(&mut self, locale: &Locale);
    fn first_char_to_lower(&mut self, locale: &Locale);
}

impl CasedString for String {
    fn first_char_case(&self, locale: &Locale) -> Option<Case> {
        self.chars().next().map(|c| locale.case(c))
    }

    fn first_char_to_upper(&mut self, locale: &Locale) {
        transform_first_char(self, |c| locale.to_title(c));
    }

    fn first_char_to_lower(&mut self, locale: &Locale) {
        transform_first_char(self, |c| locale.to_lower(c));
    }
}

fn transform_first_char(string: &mut String, transform: impl FnOnce(&str) -> String) {
    let mut indices = string.char_indices();

    indices.next();

    if let Some((i, _)) = indices.next() {
        let mut new_string = transform(&string[..i]);

        new_string.push_str(&string[i..]);

        *string = new_string;
    }
}

#[cfg(test)]
mod tests {
    use super::{CasedString, Locale};

    #[test]
    fn test_upper_string() {
        let locale: Locale = "".parse().unwrap();
        let mut string = "Some good Weather".to_owned();

        string.first_char_to_upper(&locale);

        assert_eq!(string, "Some good Weather");

        string.first_char_to_lower(&locale);

        assert_eq!(string, "some good Weather");
    }

    #[test]
    fn test_lower_string() {
        let locale: Locale = "".parse().unwrap();
        let mut string = "some good Weather".to_owned();

        string.first_char_to_lower(&locale);

        assert_eq!(string, "some good Weather");

        string.first_char_to_upper(&locale);

        assert_eq!(string, "Some good Weather");
    }
}
