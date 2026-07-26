use crate::person::Person;

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Stakeholders are all those individuals, organizations or entities who have an interest in the project.
/// Their interest could be constructive or destructive.
pub enum Stakeholder {
    /// A person who has an interest in the project.
    Individual {
        /// The personal information of the individual.
        person: Person,
        /// A description of the individual's interest in the project.
        description: Option<String>,
    },
    /// An organization that has an interest in the project.
    Organization {
        /// The name of the organization.
        name: String,
        /// A description of the organization's interest in the project.
        description: Option<String>,
    },
}

impl Stakeholder {
    /// Creates a new individual stakeholder.
    ///
    /// # Example
    /// ```
    /// use planter_core::{person::Person, stakeholders::Stakeholder};
    ///
    /// let person = Person::new("Margherita", "Hack").unwrap();
    /// let stakeholder = Stakeholder::individual(person, Some("Astrophysicist".into()));
    /// assert_eq!(stakeholder.description(), Some("Astrophysicist"));
    /// ```
    #[must_use]
    pub const fn individual(person: Person, description: Option<String>) -> Self {
        Stakeholder::Individual {
            person,
            description,
        }
    }

    /// Creates a new organization stakeholder.
    ///
    /// # Example
    /// ```
    /// use planter_core::stakeholders::Stakeholder;
    ///
    /// let stakeholder = Stakeholder::organization("Acme", Some("They buy stimpacks".into()));
    /// assert_eq!(stakeholder.description(), Some("They buy stimpacks"));
    /// ```
    #[must_use]
    pub fn organization(name: impl Into<String>, description: Option<String>) -> Self {
        Stakeholder::Organization {
            name: name.into(),
            description,
        }
    }

    /// Returns the description of the stakeholder.
    ///
    /// # Example
    /// ```
    /// use planter_core::{person::Person, stakeholders::Stakeholder};
    ///
    /// let person = Person::new("Margherita", "Hack").unwrap();
    /// let stakeholder = Stakeholder::Individual {
    ///     person,
    ///     description: Some("Astrophysicist".to_owned()),
    /// };
    /// assert_eq!(stakeholder.description(), Some("Astrophysicist"));
    /// ```
    #[must_use]
    pub fn description(&self) -> Option<&str> {
        match self {
            Stakeholder::Individual { description, .. }
            | Stakeholder::Organization { description, .. } => description.as_deref(),
        }
    }

    /// Returns a reference to the person if this is an `Individual` stakeholder.
    ///
    /// # Example
    ///
    /// ```
    /// use planter_core::{person::Person, stakeholders::Stakeholder};
    ///
    /// let person = Person::new("Margherita", "Hack").unwrap();
    /// let stakeholder = Stakeholder::Individual { person, description: None };
    /// assert!(stakeholder.person().is_some());
    ///
    /// let org = Stakeholder::Organization { name: "Acme".into(), description: None };
    /// assert!(org.person().is_none());
    /// ```
    #[must_use]
    pub const fn person(&self) -> Option<&Person> {
        match self {
            Stakeholder::Individual { person, .. } => Some(person),
            Stakeholder::Organization { .. } => None,
        }
    }

    /// Returns the name if this is an `Organization` stakeholder.
    ///
    /// # Example
    ///
    /// ```
    /// use planter_core::{person::Person, stakeholders::Stakeholder};
    ///
    /// let person = Person::new("Margherita", "Hack").unwrap();
    /// let stakeholder = Stakeholder::Individual { person, description: None };
    /// assert!(stakeholder.organization_name().is_none());
    ///
    /// let org = Stakeholder::Organization { name: "Acme".into(), description: None };
    /// assert_eq!(org.organization_name(), Some("Acme"));
    /// ```
    #[must_use]
    pub fn organization_name(&self) -> Option<&str> {
        match self {
            Stakeholder::Individual { .. } => None,
            Stakeholder::Organization { name, .. } => Some(name),
        }
    }
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use crate::person::Person;
    use crate::person::test_utils::valid_name;
    use crate::stakeholders::Stakeholder;

    fn description() -> impl Strategy<Value = Option<String>> {
        prop::option::of(r"[a-zA-Z0-9 ]{0,100}")
    }

    proptest! {
        #[test]
        fn individual_roundtrip(first in valid_name(), last in valid_name(), desc in description()) {
            let person = Person::new(&first, &last).unwrap();
            let s = Stakeholder::Individual { person, description: desc.clone() };
            let Stakeholder::Individual { person: p, description: d } = s else { panic!() };
            assert_eq!(p.full_name(), format!("{} {}", first, last));
            assert_eq!(d, desc);
        }

        #[test]
        fn organization_roundtrip(org_name in valid_name(), desc in description()) {
            let s = Stakeholder::Organization { name: org_name.clone(), description: desc.clone() };
            let Stakeholder::Organization { name: n, description: d } = s else { panic!() };
            assert_eq!(n, org_name);
            assert_eq!(d, desc);
        }

        #[test]
        fn person_accessor_returns_some_for_individual(first in valid_name(), last in valid_name()) {
            let person = Person::new(&first, &last).unwrap();
            let s = Stakeholder::Individual { person: person.clone(), description: None };
            assert_eq!(s.person(), Some(&person));
        }

        #[test]
        fn person_accessor_returns_none_for_organization(org_name in valid_name()) {
            let s = Stakeholder::Organization { name: org_name, description: None };
            assert!(s.person().is_none());
        }

        #[test]
        fn organization_name_accessor_returns_some_for_organization(org_name in valid_name()) {
            let s = Stakeholder::Organization { name: org_name.clone(), description: None };
            assert_eq!(s.organization_name(), Some(org_name.as_str()));
        }

        #[test]
        fn organization_name_accessor_returns_none_for_individual(first in valid_name(), last in valid_name()) {
            let person = Person::new(&first, &last).unwrap();
            let s = Stakeholder::Individual { person, description: None };
            assert!(s.organization_name().is_none());
        }
    }
}
