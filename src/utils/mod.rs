pub mod field_aggregator;
pub mod logical;

pub use field_aggregator::*;
pub use logical::*;

pub trait SwapResult<T, E> {
    fn swap(self) -> Result<E, T>;
}

impl<T, E> SwapResult<T, E> for Result<T, E> {
    fn swap(self) -> Result<E, T> {
        match self {
            Ok(t) => Err(t),
            Err(e) => Ok(e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn _swaps_result_ok_and_err() {
        let result: Result<usize, ()> = Ok(42);
        assert_eq!(result.swap(), Err(42));

        let result: Result<(), usize> = Err(42);
        assert_eq!(result.swap(), Ok(42));
    }
}
