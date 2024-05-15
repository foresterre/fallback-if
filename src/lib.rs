/// Fallback to an alternative, if the previous result outcome was a fail, and a predicate is true.
pub trait FallbackIf<R> {
    /// Fallback to an alternative when an outcome is considered a **fail** and **the predicate evaluates
    /// to true**, otherwise keep the current result.
    fn fallback_if<P, F>(self, predicate: P, alternative: F) -> R
    where
        P: Into<bool>,
        F: FnOnce() -> R;
}

impl<T, E> FallbackIf<Result<T, E>> for Result<T, E> {
    /// Fallback to an alternative when a result **produces an error** and **the predicate evaluates
    /// to true**, otherwise keep the current result.
    fn fallback_if<P, F>(self, predicate: P, alternative: F) -> Result<T, E>
    where
        P: Into<bool>,
        F: FnOnce() -> Result<T, E>,
    {
        if self.is_err() && predicate.into() {
            alternative()
        } else {
            self
        }
    }
}
impl<T> FallbackIf<Option<T>> for Option<T> {
    fn fallback_if<P, F>(self, predicate: P, alternative: F) -> Option<T>
    where
        P: Into<bool>,
        F: FnOnce() -> Option<T>,
    {
        if self.is_none() && predicate.into() {
            alternative()
        } else {
            self
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[yare::parameterized(
        ok =  { Ok(1), Ok(9), Ok(1) },                  // Does not enter the alternative, because input = Ok
        err_to_ok = { Err(()), Ok(9), Ok(9)  },         // Does enter the alternative, because input = Err and fallback pred. = true
        err_to_err = { Err(()), Err(()), Err(())  },    // Does enter the alternative, because input = Err and fallback pred. = true
    )]
    fn result_do_fallback(
        result: Result<u32, ()>,
        fallback_result: Result<u32, ()>,
        expected: Result<u32, ()>,
    ) {
        let outcome = result.fallback_if(true, || fallback_result);

        assert_eq!(outcome, expected)
    }

    #[yare::parameterized(
        ok =  { Ok(1), Ok(9), Ok(1) },                  // Does not enter the alternative, because input = Ok
        err_to_ok = { Err(()), Ok(9), Err(())  },       // Does not enter the alternative, because while input = Err, fallback pred. = false
        err_to_err = { Err(()), Err(()), Err(())  },    // Does not enter the alternative, because while input = Err, fallback pred. = false
    )]
    fn result_do_not_fallback(
        result: Result<u32, ()>,
        fallback_result: Result<u32, ()>,
        expected: Result<u32, ()>,
    ) {
        let outcome = result.fallback_if(false, || fallback_result);

        assert_eq!(outcome, expected)
    }

    #[yare::parameterized(
        ok =  { Some(1), Some(9), Some(1) },        // Does not enter the alternative, because input = Some
        none_to_some = { None, Some(9), Some(9)  }, // Does enter the alternative, because input = None and fallback pred. = true
        none_to_none = { None, None, None  },       // Does enter the alternative, because input = None and fallback pred. = true
    )]
    fn option_do_fallback(result: Option<u32>, fallback_value: Option<u32>, expected: Option<u32>) {
        let outcome = result.fallback_if(true, || fallback_value);

        assert_eq!(outcome, expected)
    }

    #[yare::parameterized(
        ok =  { Some(1), Some(9), Some(1) },        // Does not enter the alternative, because input = Some
        none_to_some = { None, Some(9), None  },    // Does not enter the alternative, because while input = None, fallback pred. = false
        none_to_none = { None, None, None  },       // Does not enter the alternative, because while input = None, fallback pred. = false
    )]
    fn option_do_not_fallback(
        result: Option<u32>,
        fallback_value: Option<u32>,
        expected: Option<u32>,
    ) {
        let outcome = result.fallback_if(false, || fallback_value);

        assert_eq!(outcome, expected)
    }
}
