use fretwire_locale::{
    Case::{Lower, Upper},
    CaseRelation::{Stable, Unstable},
    Locale,
};
use std::{
    borrow::Cow::{self, Borrowed, Owned},
    iter::repeat_n,
};

pub struct StateMachine<'a> {
    locale: &'a Locale,
    one_paragraph: bool,

    lower_lines: Vec<String>,
    upper_lines: Vec<String>,
    stable_lines: Vec<String>,

    leading_count: u8,
    body_count: usize,
    trailing_count: u8,
}

impl StateMachine<'_> {
    #[must_use]
    pub const fn new(locale: &Locale, one_paragraph: bool) -> StateMachine<'_> {
        StateMachine {
            locale,
            one_paragraph,

            lower_lines: Vec::new(),
            upper_lines: Vec::new(),
            stable_lines: Vec::new(),

            leading_count: 0,
            body_count: 0,
            trailing_count: 0,
        }
    }

    pub fn feed(&mut self, line: String) -> impl Iterator<Item = Cow<'static, str>> {
        self.feed_option(line).into_iter().flatten()
    }

    fn feed_option(
        &mut self,
        mut line: String,
    ) -> Option<impl Iterator<Item = Cow<'static, str>>> {
        line.truncate(line.trim_end().len());

        if let Some(character) = line.chars().next() {
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

            match self.locale.case_relation(character) {
                Unstable(Lower) => self.lower_lines.push(line),
                Unstable(Upper) => self.upper_lines.push(line),
                Stable => self.stable_lines.push(line),
            }

            result
        } else {
            if self.body_count > 0 && self.trailing_count < 2 && !self.one_paragraph {
                self.trailing_count += 1;
            }

            None
        }
    }

    pub fn flush(mut self) -> impl Iterator<Item = Cow<'static, str>> + use<> {
        let result = if self.body_count > 0 {
            Some(self.flush_not_empty())
        } else {
            None
        };

        result.into_iter().flatten()
    }

    fn flush_not_empty(&mut self) -> impl Iterator<Item = Cow<'static, str>> + use<> {
        if self.upper_lines.len() >= self.lower_lines.len() {
            for line in &mut self.lower_lines {
                self.locale.change_first_char_case(line, Upper);
            }
        } else {
            for line in &mut self.upper_lines {
                self.locale.change_first_char_case(line, Lower);
            }
        }

        let mut result = Vec::with_capacity(self.body_count);

        for vector in [
            &mut self.lower_lines,
            &mut self.upper_lines,
            &mut self.stable_lines,
        ] {
            for line in vector.drain(..) {
                result.push(Owned(line));
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
    use super::{Cow, Locale, StateMachine};
    use arbtest::arbtest;

    fn format(
        lines: impl IntoIterator<Item = String>,
        one_paragraph: bool,
    ) -> Vec<Cow<'static, str>> {
        let locale: Locale = "".parse().unwrap();
        let mut machine = StateMachine::new(&locale, one_paragraph);

        let mut result = Vec::new();
        for line in lines {
            result.extend(machine.feed(line));
        }

        result.extend(machine.flush());
        result
    }

    #[test]
    fn test_idempotence() {
        arbtest(|u| {
            let lines: Vec<String> = u.arbitrary()?;
            let one_paragraph = u.arbitrary()?;
            let first_result = format(lines, one_paragraph);
            let second_result = format(
                first_result.clone().into_iter().map(Cow::into_owned),
                one_paragraph,
            );

            assert_eq!(first_result, second_result);

            Ok(())
        });
    }

    #[test]
    fn test_empty_lines_with_many_paragraphs() {
        arbtest(|u| {
            let lines: Vec<String> = u.arbitrary()?;
            let result = format(lines, false);

            let mut streak = 0;

            for line in &result {
                if line.is_empty() {
                    streak += 1;

                    assert!(streak <= 2);
                } else {
                    streak = 0;
                }
            }

            assert!(result.first().is_none_or(|s| !s.is_empty()));
            assert!(result.last().is_none_or(|s| !s.is_empty()));

            Ok(())
        });
    }

    #[test]
    fn test_empty_lines_with_one_paragraph() {
        arbtest(|u| {
            let lines: Vec<String> = u.arbitrary()?;
            let result = format(lines, true);

            assert!(result.iter().all(|item| !item.is_empty()));

            Ok(())
        });
    }

    #[test]
    fn test_line_count() {
        arbtest(|u| {
            let lines: Vec<String> = u.arbitrary()?;
            let one_paragraph = u.arbitrary()?;
            let length = lines.len();
            let result = format(lines, one_paragraph);

            assert!(length >= result.len());

            Ok(())
        });
    }

    #[test]
    fn test_loop() {
        for one_paragraph in [false, true] {
            let locale: Locale = "uk-UA".parse().unwrap();
            let mut machine = StateMachine::new(&locale, one_paragraph);

            let mut result = Vec::new();
            for line in [
                "",
                "Перший рядок   ",
                "second line\n\r",
                "Another  ",
                "another",
                "3 three\r\n",
                "   ",
                "",
                "",
                "\n",
                "",
                "x",
                "",
                "a",
                "B",
                "",
                "a X",
                "Є d",
                "b   ",
                "   ",
                "\n",
            ] {
                result.extend(machine.feed(line.into()));
            }

            assert_eq!(result.len(), if one_paragraph { 0 } else { 10 });

            result.extend(machine.flush());

            let expected = if one_paragraph {
                vec![
                    "3 three",
                    "є d",
                    "перший рядок",
                    "a",
                    "a X",
                    "another",
                    "b",
                    "second line",
                    "x",
                ]
            } else {
                vec![
                    "3 three",
                    "Перший рядок",
                    "Another",
                    "Second line",
                    "",
                    "",
                    "x",
                    "",
                    "A",
                    "B",
                    "",
                    "є d",
                    "a X",
                    "b",
                ]
            };

            assert_eq!(result, expected);
        }
    }
}
