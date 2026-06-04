use self::row::Row;
use crate::{
    case::Case::{Lower, Neutral, Upper},
    locale::Locale,
};
use std::borrow::Cow::{self, Borrowed, Owned};

mod row;

pub struct Paragraph<'a> {
    lower_rows: Vec<Row>,
    upper_rows: Vec<Row>,
    neutral_rows: Vec<Row>,
    is_delimited: bool,
    locale: Locale<'a>,
}

impl Paragraph<'_> {
    pub const fn new(locale: Locale) -> Paragraph {
        Paragraph {
            lower_rows: Vec::new(),
            upper_rows: Vec::new(),
            neutral_rows: Vec::new(),
            is_delimited: false,
            locale,
        }
    }

    pub fn feed(&mut self, string: String) -> Vec<Cow<'static, str>> {
        let row: Row = string.into();

        if let Some(case) = row.case(&self.locale) {
            match case {
                Lower => self.lower_rows.push(row),
                Upper => self.upper_rows.push(row),
                Neutral => self.neutral_rows.push(row),
            }

            Vec::new()
        } else {
            self.is_delimited = true;

            self.flush()
        }
    }

    pub fn flush(&mut self) -> Vec<Cow<'static, str>> {
        if self.upper_rows.len() >= self.lower_rows.len() {
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
                result.push(Owned(row.into()));
            }
        }

        result.sort_by(|a, b| self.locale.compare(a, b));

        if self.is_delimited {
            result.push(Borrowed(""));
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
    use super::{Locale, Paragraph};
    use arbtest::arbtest;

    #[test]
    fn test_idempotence() {
        arbtest(|u| {
            let locale: Locale = "".parse().unwrap();
            let mut paragraph = Paragraph::new(locale);
            let lines: Vec<String> = u.arbitrary()?;

            let mut first_result: Vec<_> = lines
                .into_iter()
                .map(|line| paragraph.feed(line))
                .flatten()
                .collect();

            first_result.extend(paragraph.flush());

            let mut second_result: Vec<_> = first_result
                .clone()
                .into_iter()
                .map(|line| paragraph.feed(line.into_owned()))
                .flatten()
                .collect();

            second_result.extend(paragraph.flush());

            assert_eq!(first_result, second_result);

            Ok(())
        });
    }

    #[test]
    fn test_loop() {
        let locale: Locale = "".parse().unwrap();
        let mut paragraph = Paragraph::new(locale);
        let mut result: Vec<_> = [
            "",
            "First line     ",
            "second line\n\r",
            "3 three\r\n",
            "Another  ",
            "another",
            "   ",
            "",
            "a X",
            "C d",
            "b   ",
        ]
        .into_iter()
        .map(|line| paragraph.feed(line.into()))
        .flatten()
        .collect();

        assert_eq!(result.len(), 8);

        result.extend(paragraph.flush());

        assert_eq!(
            result,
            vec![
                "",
                "3 three",
                "Another",
                "Another",
                "First line",
                "Second line",
                "",
                "",
                "a X",
                "b",
                "c d",
            ]
        );
    }
}
