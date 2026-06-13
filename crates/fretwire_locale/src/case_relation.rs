use crate::Case;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CaseRelation {
    Stable,
    Unstable(Case),
}
