#[derive(Debug, PartialEq, Eq)]
pub enum Role {
    User,
    Vip,
    Admin,
}

impl Role {
    pub fn from_id(id: i32) -> Option<Self> {
        match id {
            1 => Some(Role::User),
            2 => Some(Role::Vip),
            3 => Some(Role::Admin),
            _ => None,
        }
    }

    pub fn to_id(&self) -> i32 {
        match self {
            Role::User => 1,
            Role::Vip => 2,
            Role::Admin => 3,
        }
    }
}