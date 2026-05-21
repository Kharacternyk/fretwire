use self::row::Row;
use crate::case::Case;
use crate::locale::Locale;
use std::borrow::Cow;

mod row;

pub struct Paragraph<'a> {
    lower_rows: Vec<Row>,
    upper_rows: Vec<Row>,
    neutral_rows: Vec<Row>,
    is_delimited: bool,
    locale: Locale<'a>,
}

impl<'a> Paragraph<'a> {
    pub fn new(locale: Locale) -> Paragraph {
        Paragraph {
            lower_rows: Vec::new(),
            upper_rows: Vec::new(),
            neutral_rows: Vec::new(),
            is_delimited: false,
            locale,
        }
    }

    pub fn feed(&mut self, string: String) -> Vec<Cow<'static, str>> {
        let row = Row::new(string);

        if let Some(case) = row.case(&self.locale) {
            match case {
                Case::Lower => self.lower_rows.push(row),
                Case::Upper => self.upper_rows.push(row),
                Case::Neutral => self.neutral_rows.push(row),
            }

            Vec::new()
        } else {
            self.is_delimited = true;

            self.flush()
        }
    }

    pub fn flush(&mut self) -> Vec<Cow<'static, str>> {
        if self.upper_rows.len() > self.lower_rows.len() {
            for row in &mut self.lower_rows {
                row.first_char_to_upper(&self.locale);
            }
        } else {
            for row in &mut self.upper_rows {
                row.first_char_to_lower(&self.locale);
            }
        }

        let capacity = self.vectors().map(|v| v.len()).sum();
        let mut result = Vec::with_capacity(capacity);

        for vector in self.vectors() {
            for row in vector.drain(..) {
                result.push(Cow::Owned(row.into()));
            }
        }

        result.sort_by(|a, b| self.locale.compare(a, b));

        if self.is_delimited {
            result.push(Cow::Borrowed(""));
            self.is_delimited = false;
        }

        result
    }

    fn vectors(&mut self) -> impl Iterator<Item = &mut Vec<Row>> {
        [
            &mut self.lower_rows,
            &mut self.upper_rows,
            &mut self.neutral_rows,
        ]
        .into_iter()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_loop() {
        let locale = crate::locale::Locale::try_new("").unwrap();
        let mut paragraph = super::Paragraph::new(locale);
        let mut result = Vec::new();

        for line in [
            "",
            "First line     ",
            "second line",
            "3 three",
            "Another  ",
            "   ",
            "",
            "a",
            "C",
            "b   ",
        ] {
            for line in paragraph.feed(line.into()) {
                result.push(line);
            }
        }

        assert_eq!(result.len(), 7);

        for line in paragraph.flush() {
            result.push(line);
        }

        assert_eq!(
            result,
            vec![
                "",
                "3 three",
                "Another",
                "First line",
                "Second line",
                "",
                "",
                "a",
                "b",
                "c",
            ]
        );
    }
}
