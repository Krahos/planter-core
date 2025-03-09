use phonenumber::PhoneNumber;
use serde_email::Email;

#[derive(Debug, Clone)]
/// Represents a person with a name and contact information.
pub struct Person {
    /// The name of the person.
    name: Name,
    /// The contact information of the person.
    contacts: Contacts,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
/// Represents the name of a person.
pub struct Name {
    /// The first name of the person.
    first: String,
    /// The last name of the person.
    last: String,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
/// The contact information available for a person. All the fields are optional.
pub struct Contacts {
    /// The email address of the person.
    email: Option<Email>,
    /// The phone number of the person.
    phone: Option<PhoneNumber>,
}

impl Person {
    /// Create a new `Person` with the given name.
    ///
    /// # Arguments
    /// * `name` - The name of the person.
    ///
    /// # Returns
    /// A new `Person` instance.
    ///
    /// # Examples
    /// ```
    /// use planter_core::person::{Person, Name};
    ///
    /// let name = Name::parse("Margherita".to_string(), "Hack".to_string()).unwrap();
    /// let person = Person::new(name);
    /// ```
    pub fn new(name: Name) -> Self {
        Person {
            name,
            contacts: Contacts {
                email: None,
                phone: None,
            },
        }
    }

    /// Add or edit the email address of the person.
    ///
    /// # Arguments
    /// * `email` - The new email address of the person.
    ///
    /// # Examples
    /// ```
    /// use planter_core::person::{Person, Name, Contacts};
    /// use serde_email::Email;
    /// use std::str::FromStr;
    ///
    /// let name = Name::parse("Margherita".to_string(), "Hack".to_string()).unwrap();
    /// let mut person = Person::new(name);
    /// let email = Email::from_str("margherita.hack@example.com").unwrap();
    /// person.add_email(email.clone());
    /// assert_eq!(person.contacts().email(), Some(&email));
    /// ```
    pub fn add_email(&mut self, email: Email) {
        self.contacts.email = Some(email);
    }

    /// Add or edit the phone number of the person.
    ///
    /// # Arguments
    /// * `phone` - The new phone number of the person.
    ///
    /// # Examples
    /// ```
    /// use planter_core::person::{Person, Name, Contacts};
    /// use serde_email::Email;
    /// use std::str::FromStr;
    /// use phonenumber::PhoneNumber;
    ///
    /// let name = Name::parse("Margherita".to_string(), "Hack".to_string()).unwrap();
    /// let mut person = Person::new(name);
    /// let phone = PhoneNumber::from_str("+1234567890").unwrap();
    /// person.add_phone(phone.clone());
    /// assert_eq!(person.contacts().phone(), Some(&phone));
    /// ```
    pub fn add_phone(&mut self, phone: PhoneNumber) {
        self.contacts.phone = Some(phone);
    }

    /// Get the contacts of the person.
    ///
    /// # Examples
    /// ```
    /// use planter_core::person::{Person, Name, Contacts};
    /// use phonenumber::PhoneNumber;
    /// use std::str::FromStr;
    ///
    /// let name = Name::parse("Margherita".to_string(), "Hack".to_string()).unwrap();
    /// let mut person = Person::new(name);
    /// let phone = PhoneNumber::from_str("+1234567890").unwrap();
    /// person.add_phone(phone.clone());
    /// assert_eq!(person.contacts().phone(), Some(&phone));
    /// ```
    pub fn contacts(&self) -> &Contacts {
        &self.contacts
    }

    /// Get the name of the person.
    ///
    /// # Examples
    /// ```
    /// use planter_core::person::{Person, Name};
    ///
    /// let name = Name::parse("Margherita".to_string(), "Hack".to_string()).unwrap();
    /// let mut person = Person::new(name.clone());
    /// assert_eq!(person.name(), &name);
    /// ```
    pub fn name(&self) -> &Name {
        &self.name
    }

    /// Get the first name of the person.
    ///
    /// # Examples
    /// ```
    /// use planter_core::person::{Person, Name};
    ///
    /// let name = Name::parse("Margherita".to_string(), "Hack".to_string()).unwrap();
    /// let mut person = Person::new(name);
    /// assert_eq!(person.first_name(), "Margherita");
    /// ```
    pub fn first_name(&self) -> &str {
        self.name.first()
    }

    /// Get the last name of the person.
    ///
    /// # Examples
    /// ```
    /// use planter_core::person::{Person, Name};
    ///
    /// let name = Name::parse("Margherita".to_string(), "Hack".to_string()).unwrap();
    /// let mut person = Person::new(name);
    /// assert_eq!(person.last_name(), "Hack");
    /// ```
    pub fn last_name(&self) -> &str {
        self.name.last()
    }
}

const NAME_LEN: usize = 50;

impl Name {
    /// Parse a name from a first and last name.
    ///
    /// # Arguments
    /// * `first` - The first name of the person.
    /// * `last` - The last name of the person.
    ///
    /// # Returns
    /// An `Option` containing the parsed `Name` if the input is valid, or
    /// `None` otherwise.
    ///
    /// # Examples
    /// ```
    /// use planter_core::person::Name;
    ///
    /// let name = Name::parse("Margherita".to_string(), "Hack".to_string());
    /// assert!(name.is_some());
    /// ```
    pub fn parse(first: String, last: String) -> Option<Self> {
        if first.is_empty() || last.is_empty() || first.len() > NAME_LEN || last.len() > NAME_LEN {
            None
        } else {
            Some(Name { first, last })
        }
    }

    /// Get the first name of the person.
    ///
    /// # Examples
    /// ```
    /// use planter_core::person::{Name};
    ///
    /// let name = Name::parse("Margherita".to_string(), "Hack".to_string()).unwrap();
    /// assert_eq!(name.first(), "Margherita");
    /// ```
    pub fn first(&self) -> &str {
        &self.first
    }

    /// Get the last name of the person.
    ///
    /// # Examples
    /// ```
    /// use planter_core::person::{Name};
    ///
    /// let name = Name::parse("Margherita".to_string(), "Hack".to_string()).unwrap();
    /// assert_eq!(name.last(), "Hack");
    /// ```
    pub fn last(&self) -> &str {
        &self.last
    }
}

impl Contacts {
    /// Create a new `Contacts`. By default, the email and phone number are set to `None`.
    ///
    /// # Returns
    /// A new `Contacts` instance.
    ///
    /// # Examples
    /// ```
    /// use planter_core::person::Contacts;
    ///
    /// let contacts = Contacts::new();
    /// assert!(contacts.email().is_none());
    /// assert!(contacts.phone().is_none());
    /// ```
    pub fn new() -> Self {
        Contacts::default()
    }

    /// Set the email address of the person.
    ///
    /// # Arguments
    /// * `email` - The email address of the person.
    ///
    /// # Returns
    /// A new `Contacts` instance with the email address set.
    ///
    /// # Examples
    /// ```
    /// use serde_email::Email;
    /// use planter_core::person::Contacts;
    /// use std::str::FromStr;
    ///
    /// let email = Email::from_str("margherita.hack@example.com").unwrap();
    /// let contacts = Contacts::new().with_email(email);
    /// assert!(contacts.email().is_some());
    /// ```
    pub fn with_email(mut self, email: Email) -> Self {
        self.email = Some(email);
        self
    }

    /// Get the email address from the contacts.
    ///
    /// # Returns
    /// An `Option` containing a reference to the email address of the person.
    ///
    /// # Examples
    /// ```
    /// use serde_email::Email;
    /// use planter_core::person::Contacts;
    ///
    /// let email = Email::from_str("margherita.hack@example.com").unwrap();
    /// let contacts = Contacts::new().with_email(email.clone());
    /// assert_eq!(contacts.email(), Some(&email));
    /// ```
    pub fn email(&self) -> Option<&Email> {
        self.email.as_ref()
    }

    /// Set the phone number of the person.
    ///
    /// # Arguments
    /// * `phone` - The phone number of the person.
    ///
    /// # Returns
    /// A new `Contacts` instance with the phone number set.
    ///
    /// # Examples
    /// ```
    /// use phonenumber::PhoneNumber;
    /// use planter_core::person::Contacts;
    /// use std::str::FromStr;
    ///
    /// let phone = PhoneNumber::from_str("+1234567890").unwrap();
    /// let contacts = Contacts::new().with_phone(phone);
    /// assert!(contacts.phone().is_some());
    /// ```
    pub fn with_phone(mut self, phone: PhoneNumber) -> Self {
        self.phone = Some(phone);
        self
    }

    /// Get the phone number from the contacts.
    ///
    /// # Returns
    /// An `Option` containing a reference to the phone number of the person.
    ///
    /// # Examples
    /// ```
    /// use phonenumber::PhoneNumber;
    /// use planter_core::person::Contacts;
    /// use std::str::FromStr;
    ///
    /// let phone = PhoneNumber::from_str("+1234567890").unwrap();
    /// let contacts = Contacts::new().with_phone(phone.clone());
    /// assert_eq!(contacts.phone().unwrap(), &phone);
    /// ```
    pub fn phone(&self) -> Option<&PhoneNumber> {
        self.phone.as_ref()
    }
}

#[cfg(test)]
#[allow(clippy::missing_panics_doc, clippy::unwrap_used)]
/// Test utilities for the `person` module.
pub mod test_utils {
    use std::str::FromStr;

    use phonenumber::PhoneNumber;
    use proptest::prelude::Strategy;
    use serde_email::Email;

    use super::{Contacts, NAME_LEN, Name};

    /// Generate a random alphabetic string of length between 1 and `NAME_LEN`.
    pub fn alphabetic_string() -> impl Strategy<Value = String> {
        ".*".prop_map(|s: String| {
            let s = s
                .chars()
                .filter(|c| c.is_alphabetic())
                .take(NAME_LEN)
                .collect::<String>();
            if s.is_empty() { String::from("a") } else { s }
        })
    }

    /// Generate a random name.
    pub fn name() -> impl Strategy<Value = Name> {
        (alphabetic_string(), alphabetic_string())
            .prop_map(|(first, last)| Name::parse(first, last).unwrap())
    }

    /// Generate a random email address.
    pub fn email() -> impl Strategy<Value = Email> {
        r"^\+?[1-9][0-9]{7,14}$".prop_map(|s: String| Email::from_str(&s).unwrap())
    }

    /// Generate a random phone number.
    pub fn phone_number() -> impl Strategy<Value = PhoneNumber> {
        r"^\d{3}-\d{3}-\d{4}$".prop_map(|s: String| PhoneNumber::from_str(&s).unwrap())
    }

    /// Generate a random contact information.
    pub fn contact() -> impl Strategy<Value = Contacts> {
        (email(), phone_number())
            .prop_map(|(email, phone)| Contacts::default().with_email(email).with_phone(phone))
    }
}

#[cfg(test)]
mod tests {
    use crate::person::test_utils::alphabetic_string;

    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_parse_name(first in alphabetic_string(), last in alphabetic_string()) {
            let name = Name::parse(first.clone(), last.clone());
            assert!(name.is_some(), "Name was {first} {last}");
        }
    }

    #[test]
    fn parse_name_returns_none_for_empty_first_name() {
        let first = String::new();
        let last = String::from("something");
        let name = Name::parse(first, last);
        assert!(name.is_none());
    }

    #[test]
    fn parse_name_returns_none_for_empty_last_name() {
        let first = String::from("something");
        let last = String::new();
        let name = Name::parse(first, last);
        assert!(name.is_none());
    }

    #[test]
    fn parse_name_returns_none_for_long_last_name() {
        let first = "a".repeat(NAME_LEN);
        let last = "b".repeat(NAME_LEN + 1);
        let name = Name::parse(first, last);
        assert!(name.is_none());
    }

    #[test]
    fn parse_name_returns_none_for_long_first_name() {
        let first = "a".repeat(NAME_LEN + 1);
        let last = "b".repeat(NAME_LEN);
        let name = Name::parse(first, last);
        assert!(name.is_none());
    }
}
