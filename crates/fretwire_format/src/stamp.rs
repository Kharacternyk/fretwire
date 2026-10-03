#[cfg(test)]
use arbitrary::Arbitrary;

#[derive(Copy, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct Stamp<'a> {
    pub marker: &'a str,
    pub value: &'a str,
}
