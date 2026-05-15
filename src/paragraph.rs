use self::row::Row;
use std::collections::HashSet;

mod row;

pub struct Paragraph {
    set: HashSet<Row>,
}
