use self::row::Row;
use crate::{
    case::Case::{Lower, Neutral, Upper},
    locale::Locale,
};
use std::{
    borrow::Cow::{self, Borrowed, Owned},
    iter::repeat_n,
};

mod row;

pub struct Paragraph<'a> {
    lower_rows: Vec<Row>,
    upper_rows: Vec<Row>,
    neutral_rows: Vec<Row>,

    leading_count: u8,
    body_count: usize,
    trailing_count: u8,

    locale: &'a Locale<'a>,
}

impl Paragraph<'_> {
    pub const fn new<'a>(locale: &'a Locale) -> Paragraph<'a> {
        Paragraph {
            lower_rows: Vec::new(),
            upper_rows: Vec::new(),
            neutral_rows: Vec::new(),

            leading_count: 0,
            body_count: 0,
            trailing_count: 0,

            locale,
        }
    }

    pub fn feed(&mut self, string: String) -> impl Iterator<Item = Cow<'static, str>> {
        self.feed_option(string).into_iter().flatten()
    }

    fn feed_option(
        &mut self,
        string: String,
    ) -> Option<impl Iterator<Item = Cow<'static, str>>> {
        let row: Row = string.into();

        if let Some(case) = row.case(self.locale) {
            let result = if self.trailing_count > 0 {
                let result = Some(self.flush_not_empty());

                self.leading_count = self.trailing_count;
                self.body_count = 1;
                self.trailing_count = 0;

                result
            } else {
                self.body_count += 1;

                None
            };

            match case {
                Lower => self.lower_rows.push(row),
                Upper => self.upper_rows.push(row),
                Neutral => self.neutral_rows.push(row),
            }

            result
        } else {
            if self.body_count > 0 && self.trailing_count < 2 {
                self.trailing_count += 1;
            }

            None
        }
    }

    pub fn flush(&mut self) -> impl Iterator<Item = Cow<'static, str>> + use<> {
        let result = if self.body_count > 0 {
            Some(self.flush_not_empty())
        } else {
            None
        };

        result.into_iter().flatten()
    }

    pub fn flush_not_empty(
        &mut self,
    ) -> impl Iterator<Item = Cow<'static, str>> + use<> {
        if self.upper_rows.len() >= self.lower_rows.len() {
            for row in &mut self.lower_rows {
                row.first_char_to_upper(self.locale);
            }
        } else {
            for row in &mut self.upper_rows {
                row.first_char_to_lower(self.locale);
            }
        }

        let mut result = Vec::with_capacity(self.body_count);

        for vector in [
            &mut self.lower_rows,
            &mut self.upper_rows,
            &mut self.neutral_rows,
        ] {
            for row in vector.drain(..) {
                result.push(Owned(row.into()));
            }
        }

        result.sort_unstable_by(|a, b| self.locale.compare(a, b));
        result.dedup_by(|a, b| self.locale.compare(a, b).is_eq());

        let leading = repeat_n(Borrowed(""), self.leading_count.into());

        leading.chain(result)
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
            let mut paragraph = Paragraph::new(&locale);
            let lines: Vec<String> = u.arbitrary()?;

            let mut first_result = Vec::new();
            for line in lines {
                first_result.extend(paragraph.feed(line));
            }

            first_result.extend(paragraph.flush());

            paragraph = Paragraph::new(&locale);

            let mut second_result = Vec::new();
            for line in first_result.clone() {
                second_result.extend(paragraph.feed(line.into_owned()));
            }

            second_result.extend(paragraph.flush());

            assert_eq!(first_result, second_result);

            Ok(())
        });
    }

    #[test]
    fn test_empty_rows() {
        arbtest(|u| {
            let locale: Locale = "".parse().unwrap();
            let mut paragraph = Paragraph::new(&locale);
            let lines: Vec<String> = u.arbitrary()?;

            let mut result = Vec::new();
            for line in lines {
                result.extend(paragraph.feed(line));
            }

            result.extend(paragraph.flush());

            let mut streak = 0;

            for line in &result {
                if line.is_empty() {
                    streak += 1;

                    assert!(streak <= 2);
                } else {
                    streak = 0
                }
            }

            assert!(result.first().map(|s| s.is_empty()) != Some(true));
            assert!(result.last().map(|s| s.is_empty()) != Some(true));

            Ok(())
        });
    }

    #[test]
    fn test_loop() {
        let locale: Locale = "".parse().unwrap();
        let mut paragraph = Paragraph::new(&locale);

        let mut result = Vec::new();
        for line in [
            "",
            "First line     ",
            "second line\n\r",
            "3 three\r\n",
            "Another  ",
            "another",
            "   ",
            "",
            "",
            "\n",
            "",
            "x",
            "",
            "a X",
            "C d",
            "b   ",
            "   ",
            "\n",
        ] {
            result.extend(paragraph.feed(line.into()));
        }

        assert_eq!(result.len(), 7);

        result.extend(paragraph.flush());

        assert_eq!(
            result,
            vec![
                "3 three",
                "Another",
                "First line",
                "Second line",
                "",
                "",
                "x",
                "",
                "a X",
                "b",
                "c d",
            ]
        );
    }
}
