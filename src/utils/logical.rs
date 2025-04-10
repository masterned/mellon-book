use core::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum Logical<T> {
    Unit(T),
    Or(Box<Logical<T>>, Box<Logical<T>>),
    And(Box<Logical<T>>, Box<Logical<T>>),
}

impl<T> Logical<T> {
    pub fn or(self, other: Self) -> Self {
        Self::Or(Box::new(self), Box::new(other))
    }

    pub fn and(self, other: Self) -> Self {
        Self::And(Box::new(self), Box::new(other))
    }
}

impl<T: fmt::Display> fmt::Display for Logical<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Logical::Unit(item) => write!(f, "{item}"),
            Logical::Or(left, right) => write!(f, "({left} or {right})"),
            Logical::And(left, right) => write!(f, "({left} and {right})"),
        }
    }
}
