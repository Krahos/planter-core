use anyhow::Context;
pub use email_address::EmailAddress;
use nutype::nutype;
pub use phonenumber::PhoneNumber;

const NAME_LEN: usize = 50;

#[derive(Debug, Clone, PartialEq, Eq)]
/// Represents a person with a name and contact information.
pub struct Person {
    /// The first name of the person.
    first_name: NameString,
    /// The last name of the person.
    last_name: NameString,
    /// The email address of the person.
    email: Option<EmailAddress>,
    /// The phone number of the person.
    phone: Option<PhoneNumber>,
}

#[nutype(
    sanitize(trim),
    validate(not_empty, len_char_max = NAME_LEN),
    derive(Debug, Eq, PartialEq, Clone, Display, Deref)
)]
pub struct NameString(String);

impl Person {
    /// Create a new `Person` with the given name and surname.
    ///
    /// # Arguments
    /// * `name` - The first name of the person.
    /// * `surname` - The surname of the person.
    ///
    /// # Returns
    /// A new `Person` instance.
    ///
    /// # Errors
    /// Returns an error if the name or surname is empty or exceeds the maximum length.
    ///
    /// # Examples
    /// ```
    /// use planter_core::person::Person;
    ///
    /// let person = Person::new("Margherita", "Hack").unwrap();
    /// ```
    pub fn new(name: impl Into<String>, surname: impl Into<String>) -> anyhow::Result<Self> {
        let name = NameString::try_new(name).context("Invalid first name")?;
        let surname = NameString::try_new(surname).context("Invalid last name")?;

        Ok(Person {
            first_name: name,
            last_name: surname,
            email: None,
            phone: None,
        })
    }

    /// Add or edit the email address of the person.
    ///
    /// # Arguments
    /// * `email` - The new email address of the person.
    ///
    /// # Examples
    /// ```
    /// use planter_core::person::Person;
    /// use email_address::EmailAddress;
    /// use std::str::FromStr;
    ///
    /// let mut person = Person::new("Margherita", "Hack").unwrap();
    /// let email = EmailAddress::from_str("margherita.hack@example.com").unwrap();
    /// person.update_email(email.clone());
    /// assert_eq!(person.email(), Some(&email));
    /// ```
    pub fn update_email(&mut self, email: EmailAddress) {
        self.email = Some(email);
    }

    /// Remove the email address of the person.
    ///
    /// # Examples
    /// ```
    /// use planter_core::person::Person;
    /// use email_address::EmailAddress;
    /// use std::str::FromStr;
    ///
    /// let mut person = Person::new("Margherita", "Hack").unwrap();
    /// let email = EmailAddress::from_str("margherita.hack@example.com").unwrap();
    /// person.update_email(email.clone());
    /// assert_eq!(person.email(), Some(&email));
    /// person.rm_email();
    /// assert!(person.email().is_none());
    /// ```
    pub fn rm_email(&mut self) {
        self.email = None;
    }

    /// Add or edit the phone number of the person.
    ///
    /// # Arguments
    /// * `phone` - The new phone number of the person.
    ///
    /// # Examples
    /// ```
    /// use planter_core::person::Person;
    /// use std::str::FromStr;
    /// use phonenumber::PhoneNumber;
    ///
    /// let mut person = Person::new("Margherita", "Hack").unwrap();
    /// let phone = PhoneNumber::from_str("+1234567890").unwrap();
    /// person.update_phone(phone.clone());
    /// assert_eq!(person.phone(), Some(&phone));
    /// ```
    pub fn update_phone(&mut self, phone: PhoneNumber) {
        self.phone = Some(phone);
    }

    /// Remove the phone number of the person.
    ///
    /// # Examples
    /// ```
    /// use planter_core::person::Person;
    /// use std::str::FromStr;
    /// use phonenumber::PhoneNumber;
    ///
    /// let mut person = Person::new("Margherita", "Hack").unwrap();
    /// let phone = PhoneNumber::from_str("+1234567890").unwrap();
    /// person.update_phone(phone.clone());
    /// assert_eq!(person.phone(), Some(&phone));
    /// person.rm_phone();
    /// assert!(person.phone().is_none());
    /// ```
    pub fn rm_phone(&mut self) {
        self.phone = None;
    }

    /// Get the phone number of the person.
    ///
    /// # Examples
    /// ```
    /// use planter_core::person::Person;
    /// use phonenumber::PhoneNumber;
    /// use std::str::FromStr;
    ///
    /// let mut person = Person::new("Margherita", "Hack").unwrap();
    /// let phone = PhoneNumber::from_str("+1234567890").unwrap();
    /// person.update_phone(phone.clone());
    /// assert_eq!(person.phone(), Some(&phone));
    /// ```
    #[must_use]
    pub const fn phone(&self) -> Option<&PhoneNumber> {
        self.phone.as_ref()
    }

    /// Get the email of the person.
    ///
    /// # Examples
    /// ```
    /// use planter_core::person::Person;
    /// use email_address::EmailAddress;
    /// use std::str::FromStr;
    ///
    /// let mut person = Person::new("Margherita", "Hack").unwrap();
    /// let email = EmailAddress::from_str("margherita.hack@example.com").unwrap();
    /// person.update_email(email.clone());
    /// assert_eq!(person.email(), Some(&email));
    /// ```
    #[must_use]
    pub const fn email(&self) -> Option<&EmailAddress> {
        self.email.as_ref()
    }

    /// Get the name of the person.
    ///
    /// # Examples
    /// ```
    /// use planter_core::person::Person;
    ///
    /// let mut person = Person::new("Margherita", "Hack").unwrap();
    /// assert_eq!(person.full_name(), "Margherita Hack");
    /// ```
    #[must_use]
    pub fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    /// Get the first name of the person.
    ///
    /// # Examples
    /// ```
    /// use planter_core::person::Person;
    ///
    /// let person = Person::new("Margherita", "Hack").unwrap();
    /// assert_eq!(person.first_name(), "Margherita");
    /// ```
    #[must_use]
    pub fn first_name(&self) -> &str {
        &self.first_name
    }

    /// Update the first name of the person.
    ///
    /// # Errors
    ///
    /// It can return an error, if the input `name` can't be converted to
    /// `NameString`
    ///
    /// # Examples
    ///
    /// ```
    /// use planter_core::person::Person;
    ///
    /// let mut person = Person::new("Margaret", "Hack").unwrap();
    /// person.update_first_name("Margherita").unwrap();
    /// assert_eq!(person.first_name(), "Margherita");
    /// ```
    pub fn update_first_name(&mut self, name: impl Into<String>) -> anyhow::Result<()> {
        self.first_name =
            NameString::try_new(name).context("Input can't be converted into NameString.")?;
        Ok(())
    }

    /// Get the last name of the person.
    ///
    /// # Examples
    /// ```
    /// use planter_core::person::Person;
    ///
    /// let mut person = Person::new("Margherita", "Hack").unwrap();
    /// assert_eq!(person.last_name(), "Hack");
    /// ```
    #[must_use]
    pub fn last_name(&self) -> &str {
        &self.last_name
    }

    /// Update the last name of the person.
    ///
    /// # Errors
    ///
    /// It can return an error, if the input `name` can't be converted to
    /// `NameString`
    ///
    /// # Examples
    ///
    /// ```
    /// use planter_core::person::Person;
    ///
    /// let mut person = Person::new("Margherita", "Hacker").unwrap();
    /// person.update_last_name("Hack").unwrap();
    /// assert_eq!(person.last_name(), "Hack");
    /// ```
    pub fn update_last_name(&mut self, name: impl Into<String>) -> anyhow::Result<()> {
        self.last_name =
            NameString::try_new(name).context("Input can't be converted into NameString.")?;
        Ok(())
    }
}

#[cfg(test)]
/// Test utilities for the `person` module.
pub mod test_utils {
    use std::str::FromStr;

    use email_address::EmailAddress;
    use phonenumber::PhoneNumber;
    use proptest::prelude::Strategy;

    /// Generate a random email address.
    pub fn email() -> impl Strategy<Value = EmailAddress> {
        r"[a-z]{1,10}@[a-z]{1,10}\.[a-z]{2,4}"
            .prop_map(|s: String| EmailAddress::from_str(&s).unwrap())
    }

    /// Generate a random phone number.
    pub fn phone_number() -> impl Strategy<Value = PhoneNumber> {
        r"\+39[0-9]{6,12}".prop_map(|s: String| PhoneNumber::from_str(&s).unwrap())
    }

    /// Generate a random valid name string (1-50 alpha chars).
    pub fn valid_name() -> impl Strategy<Value = String> {
        "[a-zA-Z]{1,50}"
    }
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use super::test_utils::{email, phone_number, valid_name};
    use crate::person::Person;

    proptest! {
        #[test]
        fn full_name_equals_first_last(first in valid_name(), last in valid_name()) {
            let person = Person::new(&first, &last).unwrap();
            assert_eq!(person.full_name(), format!("{} {}", first, last));
        }

        #[test]
        fn update_first_name_roundtrip(first in valid_name(), last in valid_name(), new_first in valid_name()) {
            let mut person = Person::new(&first, &last).unwrap();
            person.update_first_name(&new_first).unwrap();
            assert_eq!(person.first_name(), new_first);
        }

        #[test]
        fn update_last_name_roundtrip(first in valid_name(), last in valid_name(), new_last in valid_name()) {
            let mut person = Person::new(&first, &last).unwrap();
            person.update_last_name(&new_last).unwrap();
            assert_eq!(person.last_name(), new_last);
        }

        #[test]
        fn update_email_roundtrip(first in valid_name(), last in valid_name(), email in email()) {
            let mut person = Person::new(&first, &last).unwrap();
            person.update_email(email.clone());
            assert_eq!(person.email(), Some(&email));
            person.rm_email();
            assert!(person.email().is_none());
        }

        #[test]
        fn update_phone_roundtrip(first in valid_name(), last in valid_name(), phone in phone_number()) {
            let mut person = Person::new(&first, &last).unwrap();
            person.update_phone(phone.clone());
            assert_eq!(person.phone(), Some(&phone));
            person.rm_phone();
            assert!(person.phone().is_none());
        }

        #[test]
        fn new_rejects_empty_name(name in valid_name()) {
            assert!(Person::new("", &name).is_err());
            assert!(Person::new(&name, "").is_err());
        }

        #[test]
        fn new_rejects_long_name(first in "[a-zA-Z]{51,100}", last in valid_name()) {
            assert!(Person::new(&first, &last).is_err());
        }

        #[test]
        fn update_first_name_rejects_invalid(name in valid_name(), bad in "[a-zA-Z]{51,100}") {
            let mut person = Person::new("valid", &name).unwrap();
            assert!(person.update_first_name(&bad).is_err());
        }

        #[test]
        fn update_last_name_rejects_invalid(name in valid_name(), bad in "[a-zA-Z]{51,100}") {
            let mut person = Person::new(&name, "valid").unwrap();
            assert!(person.update_last_name(&bad).is_err());
        }
    }
}
