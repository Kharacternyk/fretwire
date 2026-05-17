use self::row::Row;
use crate::case::Case;
use crate::locale::Locale;

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

    pub fn feed(&mut self, string: String) -> Vec<String> {
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

    pub fn flush(&mut self) -> Vec<String> {
        if self.upper_rows.len() > self.lower_rows.len() {
            for row in &mut self.lower_rows {
                row.first_char_to_upper(&self.locale);
            }
        } else {
            for row in &mut self.upper_rows {
                row.first_char_to_lower(&self.locale);
            }
        }

        /* FIXME: this does not preallocate even though we know the final length */
        let mut result: Vec<String> = [
            &mut self.lower_rows,
            &mut self.upper_rows,
            &mut self.neutral_rows,
        ]
        .into_iter()
        .map(|vector| vector.drain(..))
        .flatten()
        .map(|row| row.into())
        .collect();

        result.sort_by(|a, b| self.locale.compare(a, b));

        if self.is_delimited {
            result.push(String::from(""));
        }

        result
    }
}
