use crate::Case;

#[derive(Debug, PartialEq, Eq)]
pub enum CaseRelation {
    Stable,
    Unstable(Case),
}
