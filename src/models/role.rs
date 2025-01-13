/// Represents the roles a user can have within the system.
#[derive(Debug, PartialEq, Eq)]
pub enum Role {
    /// A regular user role.
    User,

    /// A VIP user role with additional privileges.
    Vip,

    /// An admin role with full access.
    Admin,
}

impl Role {
    /// Converts a numeric ID to a `Role`.
    ///
    /// # Arguments
    /// * `id` - The numeric ID of the role.
    ///
    /// # Returns
    /// An `Option<Role>` corresponding to the ID, or `None` if the ID is invalid.
    pub fn from_id(id: i32) -> Option<Self> {
        match id {
            1 => Some(Role::User),
            2 => Some(Role::Vip),
            3 => Some(Role::Admin),
            _ => None,
        }
    }

    /// Converts a `Role` to its numeric ID.
    ///
    /// # Returns
    /// An `i32` representing the role ID.
    pub fn to_id(&self) -> i32 {
        match self {
            Role::User => 1,
            Role::Vip => 2,
            Role::Admin => 3,
        }
    }
}
