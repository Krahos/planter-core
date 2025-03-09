use crate::person::Person;

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
