use crate::{
    MarkerBoundary::{self, Absent, Closed, Open},
    Stamp,
};
use fretwire_locale::{
    Case::{Lower, Upper},
    CaseRelation::{Stable, Unstable},
    Locale,
};
use std::{
    borrow::Cow::{self, Borrowed, Owned},
    cmp::Ordering::{Equal, Greater, Less},
    iter::repeat_n,
};

pub struct StateMachine<'a, 'b> {
    locale: &'a Locale,
    stamp: Stamp<'b>,
    one_paragraph: bool,

    lower_lines: Vec<String>,
    upper_lines: Vec<String>,
    stable_lines: Vec<String>,

    leading_count: u8,
    body_count: usize,
    trailing_count: u8,
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
        let mut add_stamps = false;

        for vector in [
            &mut self.lower_lines,
            &mut self.upper_lines,
            &mut self.stable_lines,
        ] {
            for line in vector.drain(..) {
                let marker_boundary = MarkerBoundary::find(&line, self.stamp.marker);

                if !add_stamps {
                    match marker_boundary {
                        Closed(_) => add_stamps = true,
                        Open(i) if i + self.stamp.marker.len() == line.len() => {
                            add_stamps = true;
                        }
                        _ => {}
                    }
                }

                result.push((line, marker_boundary));
            }
        }

        let compare_before_marker =
            |a: &(String, MarkerBoundary), b: &(String, MarkerBoundary)| {
                self.locale
                    .compare(a.1.before_marker(&a.0), b.1.before_marker(&b.0))
            };

        result.sort_unstable_by(|a, b| match compare_before_marker(a, b) {
            Equal => match (a.1, b.1) {
                (Closed(i), Closed(j)) => self.locale.compare(&a.0[i..], &b.0[j..]),
                (Closed(_), _) => Less,
                (_, Closed(_)) => Greater,
                _ => a.0.len().cmp(&b.0.len()),
            },
            x => x,
        });
        result.dedup_by(|a, b| match compare_before_marker(a, b) {
            Equal => add_stamps || a.0.len() == b.0.len(),
            _ => false,
        });

        if add_stamps {
            for (line, boundary) in &mut result {
                match boundary {
                    Absent => {
                        line.push_str(self.stamp.marker);
                        line.push_str(self.stamp.value);
                    }
                    Open(i) => {
                        line.push_str(&self.stamp.marker[line.len() - *i..]);
                        line.push_str(self.stamp.value);
                    }
                    Closed(_) => {}
                }
            }
        }

        let leading = repeat_n(Borrowed(""), self.leading_count.into());

        leading.chain(result.into_iter().map(|(line, _)| Owned(line)))
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
    fn test_marker_boundary_after_case_expansion() {
        let locale: Locale = "tr".parse().unwrap();
        let stamp = Stamp {
            marker: "@",
            value: "2026",
        };
        let mut machine = StateMachine::new(&locale, stamp, false);

        let mut result = Vec::new();
        for line in ["i@", "A@"] {
            result.extend(machine.feed(line.into()));
        }
        result.extend(machine.flush());

        assert_eq!(result, ["A@2026", "İ@2026"]);
    }

    #[test]
    fn test_idempotence_after_case_changes_partial_marker() {
        let stamp = Stamp {
            marker: "qx",
            value: "2026",
        };
        let first_result = format(["q".into(), "E".into()], stamp, false);
        let second_result = format(
            first_result.clone().into_iter().map(Cow::into_owned),
            stamp,
            false,
        );

        assert_eq!(first_result, ["E", "Q"]);
        assert_eq!(first_result, second_result);
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
            value: "12",
        };

        for one_paragraph in [false, true] {
            let mut machine = StateMachine::new(&locale, stamp, one_paragraph);

            let mut result = Vec::new();
            for line in [
                "",
                "Перший рядок   ",
                "Another.  ",
                "another     ",
                "second line\n\r",
                "Another ",
                "3 three\r\n",
                "another.",
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
                "b. :",
                "Є d",
                "b",
                "b. :678",
                "b. ",
                "b. :34   ",
                "b. :92",
                "   ",
                "\n",
            ] {
                result.extend(machine.feed(line.into()));
            }

            assert_eq!(result.len(), if one_paragraph { 0 } else { 11 });

            result.extend(machine.flush());

            let expected = if one_paragraph {
                vec![
                    "3 three. :12",
                    "є d. :12",
                    "перший рядок. :12",
                    "a X. :12",
                    "another. :12",
                    "b. :34",
                    "second line. :12",
                    "x. :12",
                ]
            } else {
                vec![
                    "3 three",
                    "Перший рядок",
                    "Another",
                    "Another.",
                    "Second line",
                    "",
                    "",
                    "x. :12",
                    "",
                    "A X",
                    "B",
                    "",
                    "є d. :12",
                    "a X. :12",
                    "b. :34",
                ]
            };

            assert_eq!(result, expected);
        }
    }
}
