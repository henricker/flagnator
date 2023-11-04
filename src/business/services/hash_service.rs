#[cfg(test)]
use mockall::{automock, predicate::*};

#[cfg_attr(test, automock)]
pub trait HashService {
    fn hash(&self, password: &str) -> String;
}
