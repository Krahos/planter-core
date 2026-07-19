use crate::person::Person;

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
/// Represents a resource that can be used in a project. A resource can be either a material or personnel.
pub enum Resource {
    /// Represents a material resource that can be used in a project.
    Material(Material),
    /// Represents a personnel resource. Personnel is usually a resource that can complete tasks
    Personnel {
        /// Information about the person.
        person: Person,
        /// Hourly rate of the person.
        hourly_rate: Option<u16>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
/// Represents a material resource that can be used in a project.
/// It can be either consumable or non-consumable.
pub enum Material {
    /// A consumable resource is a material that needs to be resupplied after use.
    Consumable(Consumable),
    /// A non-consumable resource is a material that does not need to be resupplied after use.
    NonConsumable(NonConsumable),
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
/// Represents a consumable material resource that can be used in a project.
pub struct Consumable {
    /// Name of the consumable material.
    name: String,
    /// Available quantity of the consumable material.
    quantity: Option<u16>,
    /// Cost to buy this material.
    cost_per_unit: Option<u16>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
/// Represents a non-consumable material resource that can be used in a project.
pub struct NonConsumable {
    /// Name of the non-consumable material.
    name: String,
    /// Available quantity of the non-consumable material.
    quantity: Option<u16>,
    /// Cost to buy this material.
    cost_per_unit: Option<u16>,
    /// Some non consumable materials can have a hourly rate. For example, due to energy consumption.
    hourly_rate: Option<u16>,
}

impl From<NonConsumable> for Consumable {
    fn from(value: NonConsumable) -> Self {
        Consumable {
            name: value.name,
            quantity: value.quantity,
            cost_per_unit: value.cost_per_unit,
        }
    }
}

impl From<Consumable> for NonConsumable {
    fn from(value: Consumable) -> Self {
        NonConsumable {
            name: value.name,
            quantity: value.quantity,
            cost_per_unit: value.cost_per_unit,
            hourly_rate: None,
        }
    }
}

impl Material {
    /// Returns a consumable material by default, with the given name.
    ///
    /// # Example
    /// ```
    /// use planter_core::resources::Material;
    ///
    /// let material = Material::new("Steel".to_owned());
    /// assert_eq!(material.name(), "Steel");
    /// ```
    #[must_use]
    pub fn new(name: impl Into<String>) -> Self {
        Material::Consumable(Consumable::new(name))
    }

    /// Returns the name of the material.
    /// # Example
    /// ```
    /// use planter_core::resources::Material;
    ///
    /// let material = Material::new("Steel".to_owned());
    /// assert_eq!(material.name(), "Steel");
    /// ```
    #[must_use]
    pub fn name(&self) -> &str {
        match self {
            Material::Consumable(consumable) => &consumable.name,
            Material::NonConsumable(non_consumable) => &non_consumable.name,
        }
    }

    /// Updates the name of the material.
    /// # Example
    /// ```
    /// use planter_core::resources::Material;
    ///
    /// let mut material = Material::new("Steel".to_owned());
    /// material.update_name("Iron".to_owned());
    /// assert_eq!(material.name(), "Iron");
    /// ```
    pub fn update_name(&mut self, name: impl Into<String>) {
        match self {
            Material::Consumable(consumable) => consumable.name = name.into(),
            Material::NonConsumable(non_consumable) => non_consumable.name = name.into(),
        }
    }

    /// Returns the quantity of materials.
    /// # Example
    /// ```
    /// use planter_core::resources::Material;
    ///
    /// let material = Material::new("Steel".to_owned());
    /// assert_eq!(material.quantity(), None);
    /// ```
    #[must_use]
    pub const fn quantity(&self) -> Option<u16> {
        match self {
            Material::Consumable(consumable) => consumable.quantity,
            Material::NonConsumable(non_consumable) => non_consumable.quantity,
        }
    }

    /// Updates the quantity of materials.
    /// # Example
    /// ```
    /// use planter_core::resources::Material;
    ///
    /// let mut material = Material::new("Steel".to_owned());
    /// material.update_quantity(3);
    /// assert_eq!(material.quantity(), Some(3));
    /// ```
    pub const fn update_quantity(&mut self, quantity: u16) {
        match self {
            Material::Consumable(consumable) => consumable.quantity = Some(quantity),
            Material::NonConsumable(non_consumable) => non_consumable.quantity = Some(quantity),
        }
    }

    /// Remove the quantity of materials.
    /// # Example
    /// ```
    /// use planter_core::resources::Material;
    ///
    /// let mut material = Material::new("Steel".to_owned());
    /// material.update_quantity(3);
    /// assert_eq!(material.quantity(), Some(3));
    /// material.remove_quantity();
    /// assert_eq!(material.quantity(), None);
    /// ```
    pub const fn remove_quantity(&mut self) {
        match self {
            Material::Consumable(consumable) => consumable.quantity = None,
            Material::NonConsumable(non_consumable) => non_consumable.quantity = None,
        }
    }
    /// Returns the cost per unit of the material.
    /// # Example
    /// ```
    /// use planter_core::resources::Material;
    ///
    /// let material = Material::new("Steel".to_owned());
    /// assert_eq!(material.cost_per_unit(), None);
    /// ```
    #[must_use]
    pub const fn cost_per_unit(&self) -> Option<u16> {
        match self {
            Material::Consumable(consumable) => consumable.cost_per_unit,
            Material::NonConsumable(non_consumable) => non_consumable.cost_per_unit,
        }
    }

    /// Updates the cost per unit of the material.
    /// # Example
    /// ```
    /// use planter_core::resources::Material;
    ///
    /// let mut material = Material::new("Steel".to_owned());
    /// material.update_cost_per_unit(3);
    /// assert_eq!(material.cost_per_unit(), Some(3));
    /// ```
    pub const fn update_cost_per_unit(&mut self, cost_per_unit: u16) {
        match self {
            Material::Consumable(consumable) => consumable.cost_per_unit = Some(cost_per_unit),
            Material::NonConsumable(non_consumable) => {
                non_consumable.cost_per_unit = Some(cost_per_unit)
            }
        }
    }

    /// Remove the cost per unit of the material.
    /// # Example
    /// ```
    /// use planter_core::resources::Material;
    ///
    /// let mut material = Material::new("Steel".to_owned());
    /// material.update_cost_per_unit(3);
    /// assert_eq!(material.cost_per_unit(), Some(3));
    /// material.remove_cost_per_unit();
    /// assert_eq!(material.cost_per_unit(), None);
    /// ```
    pub const fn remove_cost_per_unit(&mut self) {
        match self {
            Material::Consumable(consumable) => consumable.cost_per_unit = None,
            Material::NonConsumable(non_consumable) => non_consumable.cost_per_unit = None,
        }
    }
}

impl Consumable {
    /// Creates a new consumable material resource with the given name.
    ///
    /// # Example
    /// ```
    /// use planter_core::resources::{Consumable, Material};
    ///
    /// let consumable = Consumable::new("Steel".to_owned());
    /// let material = Material::Consumable(consumable);
    /// assert_eq!(material.name(), "Steel");
    /// ```
    #[must_use]
    pub fn new(name: impl Into<String>) -> Self {
        Consumable {
            name: name.into(),
            quantity: None,
            cost_per_unit: None,
        }
    }
}

impl NonConsumable {
    /// Creates a new non-consumable material resource with the given name.
    ///
    /// # Example
    /// ```
    /// use planter_core::resources::{NonConsumable, Material};
    ///
    /// let non_consumable = NonConsumable::new("Drill".to_owned());
    /// let material = Material::NonConsumable(non_consumable);
    /// assert_eq!(material.name(), "Drill");
    /// ```
    #[must_use]
    pub fn new(name: impl Into<String>) -> Self {
        NonConsumable {
            name: name.into(),
            quantity: None,
            hourly_rate: None,
            cost_per_unit: None,
        }
    }

    /// Returns the hourly rate of the non-consumable material.
    /// # Example
    /// ```
    /// use planter_core::resources::NonConsumable;
    ///
    /// let non_consumable = NonConsumable::new("Steel".to_owned());
    /// assert_eq!(non_consumable.hourly_rate(), None);
    /// ```
    #[must_use]
    pub const fn hourly_rate(&self) -> Option<u16> {
        self.hourly_rate
    }

    /// Updates the hourly_rate of the non consumable material.
    /// # Example
    /// ```
    /// use planter_core::resources::NonConsumable;
    ///
    /// let mut non_consumable = NonConsumable::new("Steel".to_owned());
    /// non_consumable.update_hourly_rate(3);
    /// assert_eq!(non_consumable.hourly_rate(), Some(3));
    /// ```
    pub const fn update_hourly_rate(&mut self, hourly_rate: u16) {
        self.hourly_rate = Some(hourly_rate);
    }

    /// Remove the cost per unit of the non consumable material.
    /// # Example
    /// ```
    /// use planter_core::resources::NonConsumable;
    ///
    /// let mut non_consumable = NonConsumable::new("Steel".to_owned());
    /// non_consumable.update_hourly_rate(3);
    /// assert_eq!(non_consumable.hourly_rate(), Some(3));
    /// non_consumable.remove_hourly_rate();
    /// assert_eq!(non_consumable.hourly_rate(), None);
    /// ```
    pub const fn remove_hourly_rate(&mut self) {
        self.hourly_rate = None;
    }
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use crate::resources::{Consumable, Material, NonConsumable};

    fn material_name() -> impl Strategy<Value = String> {
        r"[a-zA-Z0-9 ]{1,30}".prop_map(|s: String| s)
    }

    fn quantity() -> impl Strategy<Value = u16> {
        1..u16::MAX
    }

    fn unit_cost() -> impl Strategy<Value = u16> {
        1..u16::MAX
    }

    proptest! {
        #[test]
        fn consumable_setters_roundtrip(name in material_name(), qty in quantity(), cost in unit_cost()) {
            let mut m = Material::Consumable(Consumable::new(name.clone()));
            assert_eq!(m.name(), name);

            assert_eq!(m.quantity(), None);
            m.update_quantity(qty);
            assert_eq!(m.quantity(), Some(qty));
            m.remove_quantity();
            assert_eq!(m.quantity(), None);

            assert_eq!(m.cost_per_unit(), None);
            m.update_cost_per_unit(cost);
            assert_eq!(m.cost_per_unit(), Some(cost));
            m.remove_cost_per_unit();
            assert_eq!(m.cost_per_unit(), None);
        }

        #[test]
        fn nonconsumable_setters_roundtrip(name in material_name(), qty in quantity(), cost in unit_cost()) {
            let mut m = Material::NonConsumable(NonConsumable::new(name.clone()));
            assert_eq!(m.name(), name);

            assert_eq!(m.quantity(), None);
            m.update_quantity(qty);
            assert_eq!(m.quantity(), Some(qty));
            m.remove_quantity();
            assert_eq!(m.quantity(), None);

            assert_eq!(m.cost_per_unit(), None);
            m.update_cost_per_unit(cost);
            assert_eq!(m.cost_per_unit(), Some(cost));
            m.remove_cost_per_unit();
            assert_eq!(m.cost_per_unit(), None);
        }

        #[test]
        fn nonconsumable_hourly_rate_roundtrip(rate in 1u16..5000u16) {
            let mut nc = NonConsumable::new("item");
            assert_eq!(nc.hourly_rate(), None);
            nc.update_hourly_rate(rate);
            assert_eq!(nc.hourly_rate(), Some(rate));
            nc.remove_hourly_rate();
            assert_eq!(nc.hourly_rate(), None);
        }

        #[test]
        fn consumable_to_nonconsumable_preserves_fields(name in material_name(), qty in quantity(), cost in unit_cost()) {
            let mut m = Material::Consumable(Consumable::new(name.clone()));
            m.update_quantity(qty);
            m.update_cost_per_unit(cost);

            let Material::Consumable(ref inner) = m else { panic!() };
            let nc = NonConsumable::from(inner.clone());

            assert_eq!(nc.name, inner.name);
            assert_eq!(nc.quantity, inner.quantity);
            assert_eq!(nc.cost_per_unit, inner.cost_per_unit);
        }

        #[test]
        fn nonconsumable_to_consumable_preserves_fields(qty in quantity(), cost in unit_cost()) {
            let mut m = Material::NonConsumable(NonConsumable::new("item"));
            m.update_quantity(qty);
            m.update_cost_per_unit(cost);
            let Material::NonConsumable(ref nc) = m else { panic!() };
            let c = Consumable::from(nc.clone());
            assert_eq!(c.quantity, Some(qty));
            assert_eq!(c.cost_per_unit, Some(cost));
        }
    }
}
