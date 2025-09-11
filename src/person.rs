use anyhow::Context;
pub use email_address::EmailAddress;
use nutype::nutype;
pub use phonenumber::PhoneNumber;

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
    /// use planter_core::person::Person;
    ///
    /// let person = Person::new("Margherita", "Hack").unwrap();
    /// ```
    pub fn new(name: impl Into<String>, surname: impl Into<String>) -> Option<Self> {
        let (name, surname) = match (NameString::try_new(name), NameString::try_new(surname)) {
            (Ok(n), Ok(s)) => (n, s),
            _ => return None,
        };

        Some(Person {
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
    /// assert_eq!(person.email(), &Some(email));
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
    /// assert_eq!(person.email(), &Some(email));
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
    /// assert_eq!(person.phone(), &Some(phone));
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
    /// assert_eq!(person.phone(), &Some(phone));
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
    /// assert_eq!(person.phone(), &Some(phone));
    /// ```
    pub fn phone(&self) -> &Option<PhoneNumber> {
        &self.phone
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
    /// assert_eq!(person.email(), &Some(email));
    /// ```
    pub fn email(&self) -> &Option<EmailAddress> {
        &self.email
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

const NAME_LEN: usize = 50;

#[cfg(test)]
/// Test utilities for the `person` module.
pub mod test_utils {
    use std::str::FromStr;

    use email_address::EmailAddress;
    use phonenumber::PhoneNumber;
    use proptest::prelude::Strategy;

    /// Generate a random email address.
    pub fn email() -> impl Strategy<Value = EmailAddress> {
        r"^\+?[1-9][0-9]{7,14}$".prop_map(|s: String| EmailAddress::from_str(&s).unwrap())
    }

    /// Generate a random phone number.
    pub fn phone_number() -> impl Strategy<Value = PhoneNumber> {
        r"^\d{3}-\d{3}-\d{4}$".prop_map(|s: String| PhoneNumber::from_str(&s).unwrap())
    }
}
