use std::panic;
use std::result::Result;

#[derive(Debug, PartialEq)]
enum Error {
    ContractError,
}

/// https://stackoverflow.com/users/1763356/veedrac
macro_rules! function {
    () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let name = type_name_of(f);
        &name[..name.len() - 3]
    }};
}

macro_rules! pre {
    ($condition:expr, $message:expr) => {
        match panic::catch_unwind(|| {
            assert!($condition);
        }) {
            Err(_) => panic!("Pre-condition of {} violated: {}", function!(), $message),
            _ => (),
        }
    };
}

/// Simple add assertion using my macro
fn add_macro(a: usize, b: usize) -> usize {
    pre!(a % 2 == 0, "a must be even");
    pre!(b % 2 == 0, "b must be even");
    a + b
}

/// For the purpose of illustration suppose this add function is only able to add even numbers
fn add(a: usize, b: usize) -> Result<usize, Error> {
    match panic::catch_unwind(|| {
        assert!(a % 2 == 0);
        assert!(b % 2 == 0);
    }) {
        Ok(_) => Ok(a + b),
        Error => Err(Error::ContractError),
    }
}

#[pre(a % 2 == 0, "a must be even")]
#[pre(b % 2 == 0, "b must be even")]
/// Same as above using the `contracts` crate
fn add_contracts(a: usize, b: usize) -> usize {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_contract_failure() {
        let result = add(2, 3);
        match result {
            Ok(_) => panic!("Expected error!"),
            Err(err) => assert_eq!(err, Error::ContractError),
        }
    }

    #[test]
    fn add_macro_contract_failure() {
        let result = add_macro(2, 3);
    }

    #[test]
    fn add_contracts_contract_failure() {
        let result = add_contracts(2, 3);
    }
}
