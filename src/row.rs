use icu_casemap::CaseMapper;
use icu_locale::Locale;

pub struct Row {
    data: String,
}

pub enum RowCase {
    Upper,
    Lower,
    Neutral,
}

impl Row {
    pub fn new(mut data: String) -> Row {
        data.truncate(data.trim_end().len());
        Row { data }
    }

    pub fn case(&self, mapper: &CaseMapper) -> RowCase {
        RowCase::Upper
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_trailing_whitespace() {
        let row = super::Row::new(String::from("abc "));
        assert_eq!(row.data, "abc");
    }
}
