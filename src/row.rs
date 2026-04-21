use crate::case::Case;
use crate::locale::Locale;

pub struct Row {
    data: String,
}

impl Row {
    pub fn new(mut data: String) -> Row {
        data.truncate(data.trim_end().len());
        Row { data }
    }

    pub fn case(&self, locale: &Locale) -> Case {
        match self.data.chars().next() {
            Some(character) => locale.case(character),
            _ => Case::Neutral,
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_trailing_whitespace() {
        let row = super::Row::new(String::from("abc \t  \n"));
        assert_eq!(row.data, "abc");
    }
}
