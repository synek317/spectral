use super::{AssertionFailure, Spec};

use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

pub trait HashMapAssertions<'s, K: Hash + Eq, V: PartialEq> {
    fn has_length(&mut self, expected: usize);
    fn contains_key(&mut self, expected_key: &K) -> Spec<'s, V>;
    fn contains_key_with_value(&mut self, expected_key: &K, expected_value: &V);
}

impl<'s, K, V> HashMapAssertions<'s, K, V> for Spec<'s, HashMap<K, V>>
    where K: Hash + Eq + Debug,
          V: PartialEq + Debug
{
    /// Asserts that the length of the subject hashmap is equal to the provided length. The subject
    /// type must be of `HashMap`.
    ///
    /// ```rust,ignore
    /// let mut test_map = HashMap::new();
    /// test_map.insert(1, 1);
    /// test_map.insert(2, 2);
    ///
    /// assert_that(&test_map).has_length(2);
    /// ```
    fn has_length(&mut self, expected: usize) {
        let subject = self.subject;

        if subject.len() != expected {
            AssertionFailure::from_spec(self)
                .with_expected(format!("hashmap to have length <{}>", expected))
                .with_actual(format!("<{}>", subject.len()))
                .fail();
        }
    }

    /// Asserts that the subject hashmap contains the expected key. The subject type must be
    /// of `HashMap`.
    ///
    /// This will return a new `Spec` containing the associated value if the key is present.
    ///
    /// ```rust,ignore
    /// let mut test_map = HashMap::new();
    /// test_map.insert("hello", "hi");
    ///
    /// assert_that(&test_map).contains_key(&"hello");
    /// ```
    fn contains_key(&mut self, expected_key: &K) -> Spec<'s, V> {
        let subject = self.subject;

        if let Some(value) = subject.get(expected_key) {
            return Spec {
                subject: value,
                description: self.description,
            };
        }

        let subject_keys: Vec<&K> = subject.keys().collect();

        AssertionFailure::from_spec(self)
            .with_expected(format!("hashmap to contain key <{:?}>", expected_key))
            .with_actual(format!("<{:?}>", subject_keys))
            .fail();

        unreachable!();
    }

    /// Asserts that the subject hashmap contains the expected key with the expected value.
    /// The subject type must be of `HashMap`.
    ///
    /// ```rust,ignore
    /// let mut test_map = HashMap::new();
    /// test_map.insert("hello", "hi");
    ///
    /// assert_that(&test_map).contains_key_with_value(&"hello", &"hi");
    /// ```
    fn contains_key_with_value(&mut self, expected_key: &K, expected_value: &V) {
        let expected_message = format!("hashmap containing key <{:?}> with value <{:?}>",
                                       expected_key,
                                       expected_value);
        let subject = self.subject;

        if let Some(value) = subject.get(expected_key) {
            if value.eq(expected_value) {
                return;
            }

            AssertionFailure::from_spec(self)
                .with_expected(expected_message)
                .with_actual(format!("key <{:?}> with value <{:?}> instead", expected_key, value))
                .fail();

            unreachable!();
        }

        let subject_keys: Vec<&K> = subject.keys().collect();

        AssertionFailure::from_spec(self)
            .with_expected(expected_message)
            .with_actual(format!("no matching key, keys are <{:?}>", subject_keys))
            .fail();

    }
}

#[cfg(test)]
mod tests {

    use super::super::prelude::*;

    use std::collections::HashMap;

    #[test]
    fn should_not_panic_if_hashmap_length_matches_expected() {
        let mut test_map = HashMap::new();
        test_map.insert(1, 1);
        test_map.insert(2, 2);

        assert_that(&test_map).has_length(2);
    }

    #[test]
    #[should_panic(expected = "\n\texpected: hashmap to have length <1>\n\t but was: <2>")]
    fn should_panic_if_hashmap_length_does_not_match_expected() {
        let mut test_map = HashMap::new();
        test_map.insert(1, 1);
        test_map.insert(2, 2);

        assert_that(&test_map).has_length(1);
    }

    #[test]
    fn should_not_panic_if_hashmap_contains_key() {
        let mut test_map = HashMap::new();
        test_map.insert("hello", "hi");

        assert_that(&test_map).contains_key(&"hello");
    }

    #[test]
    // Unfortunately the order of the keys can change. Doesn't seem to make sense to sort them
    // just for the sake of checking the panic message.
    #[should_panic]
    fn should_not_panic_if_hashmap_does_not_contain_key() {
        let mut test_map = HashMap::new();
        test_map.insert("hi", "hi");
        test_map.insert("hey", "hey");

        assert_that(&test_map).contains_key(&"hello");
    }

    #[test]
    fn should_be_able_to_chain_value_from_contains_key() {
        let mut test_map = HashMap::new();
        test_map.insert("hello", "hi");

        assert_that(&test_map).contains_key(&"hello").is_equal_to(&"hi");
    }

    #[test]
    fn should_not_panic_if_hashmap_contains_key_with_value() {
        let mut test_map = HashMap::new();
        test_map.insert("hello", "hi");

        assert_that(&test_map).contains_key_with_value(&"hello", &"hi");
    }

    #[test]
    #[should_panic(expected = "\n\texpected: hashmap containing key <\"hey\"> with value <\"hi\">\
                   \n\t but was: no matching key, keys are <[\"hello\"]>")]
    fn should_panic_if_hashmap_contains_key_with_value_without_key() {
        let mut test_map = HashMap::new();
        test_map.insert("hello", "hi");

        assert_that(&test_map).contains_key_with_value(&"hey", &"hi");
    }

    #[test]
    #[should_panic(expected = "\n\texpected: hashmap containing key <\"hi\"> with value <\"hey\">\
                   \n\t but was: key <\"hi\"> with value <\"hello\"> instead")]
    fn should_panic_if_hashmap_contains_key_with_value_with_different_value() {
        let mut test_map = HashMap::new();
        test_map.insert("hi", "hello");

        assert_that(&test_map).contains_key_with_value(&"hi", &"hey");
    }
}