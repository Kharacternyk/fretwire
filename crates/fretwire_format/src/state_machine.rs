use crate::{MarkerBoundary, Stamp};
use fretwire_locale::{
    Case::{Lower, Upper},
    CaseRelation::{Stable, Unstable},
    Locale,
};
use std::{
    borrow::Cow::{self, Borrowed, Owned},
    iter::repeat_n,
};

pub struct StateMachine<'a, 'b> {
    locale: &'a Locale,
    stamp: Stamp<'b>,
    one_paragraph: bool,

    lower_lines: Vec<(String, Option<MarkerBoundary>)>,
    upper_lines: Vec<(String, Option<MarkerBoundary>)>,
    stable_lines: Vec<(String, Option<MarkerBoundary>)>,

    leading_count: u8,
    body_count: usize,
    trailing_count: u8,

    add_stamps: bool,
}

impl<'a, 'b> StateMachine<'a, 'b> {
    #[must_use]
    pub const fn new(locale: &'a Locale, stamp: Stamp<'b>, one_paragraph: bool) -> Self {
        Self {
            locale,
            stamp,
            one_paragraph,

            lower_lines: Vec::new(),
            upper_lines: Vec::new(),
            stable_lines: Vec::new(),

            leading_count: 0,
            body_count: 0,
            trailing_count: 0,

            add_stamps: false,
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
                self.add_stamps = false;

                result
            } else {
                self.body_count += 1;

                None
            };

            let marker_boundary = MarkerBoundary::find(&line, &self.stamp.marker);

            if let Some(_) = marker_boundary {
                self.add_stamps = true;
            }

            let item = (line, marker_boundary);

            match self.locale.case_relation(character) {
                Unstable(Lower) => self.lower_lines.push(item),
                Unstable(Upper) => self.upper_lines.push(item),
                Stable => self.stable_lines.push(item),
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
            for (line, _) in &mut self.lower_lines {
                self.locale.change_first_char_case(line, Upper);
            }
        } else {
            for (line, _) in &mut self.upper_lines {
                self.locale.change_first_char_case(line, Lower);
            }
        }

        let mut result = Vec::with_capacity(self.body_count);

        for vector in [
            &mut self.lower_lines,
            &mut self.upper_lines,
            &mut self.stable_lines,
        ] {
            for item in vector.drain(..) {
                result.push(item);
            }
        }

        result.sort_unstable_by(
            |(first_line, first_boundary), (second_line, second_boundary)| {
                self.locale.compare(first_line, second_line)
            },
        );
        result.dedup_by(|a, b| self.locale.compare(a, b).is_eq());

        let leading = repeat_n(Borrowed(""), self.leading_count.into());

        leading.chain(result)
    }
}

#[cfg(test)]
mod tests {
    use super::{Cow, Locale, Stamp, StateMachine};
    use arbtest::arbtest;

    fn format(
        lines: impl IntoIterator<Item = String>,
        stamp: Stamp,
        one_paragraph: bool,
    ) -> Vec<Cow<'static, str>> {
        let locale: Locale = "".parse().unwrap();
        let mut machine = StateMachine::new(&locale, stamp, one_paragraph);

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
            let stamp = u.arbitrary()?;
            let first_result = format(lines, stamp, one_paragraph);
            let second_result = format(
                first_result.clone().into_iter().map(Cow::into_owned),
                stamp,
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
            let stamp = u.arbitrary()?;
            let result = format(lines, stamp, false);

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
            let stamp = u.arbitrary()?;
            let result = format(lines, stamp, true);

            assert!(result.iter().all(|item| !item.is_empty()));

            Ok(())
        });
    }

    #[test]
    fn test_line_count() {
        arbtest(|u| {
            let lines: Vec<String> = u.arbitrary()?;
            let stamp = u.arbitrary()?;
            let one_paragraph = u.arbitrary()?;
            let length = lines.len();
            let result = format(lines, stamp, one_paragraph);

            assert!(length >= result.len());

            Ok(())
        });
    }

    #[test]
    fn test_loop() {
        let locale: Locale = "uk-UA".parse().unwrap();
        let stamp = Stamp {
            marker: ". :",
            value: "12-34",
        };

        for one_paragraph in [false, true] {
            let mut machine = StateMachine::new(&locale, stamp, one_paragraph);

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
                "x. :",
                "",
                "a X",
                "B",
                "",
                "a X.",
                "Є d",
                "b. :67-89",
                "   ",
                "\n",
            ] {
                result.extend(machine.feed(line.into()));
            }

            assert_eq!(result.len(), if one_paragraph { 0 } else { 10 });

            result.extend(machine.flush());

            let expected = if one_paragraph {
                vec![
                    "3 three. :12-34",
                    "є d. :12-34",
                    "перший рядок. :12-34",
                    "a X. :12-34",
                    "another. :12-34",
                    "b. :12-34",
                    "b. :67-89",
                    "second line. :12-34",
                    "x. :12-34",
                ]
            } else {
                vec![
                    "3 three",
                    "Перший рядок",
                    "Another",
                    "Second line",
                    "",
                    "",
                    "x. :12-34",
                    "",
                    "A X",
                    "B",
                    "",
                    "є d. :12-34",
                    "a X. :12-34",
                    "b. :67-89",
                ]
            };

            assert_eq!(result, expected);
        }
    }
}
