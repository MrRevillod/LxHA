
use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};
use crate::models::user::{Role, User};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Token {
    pub token: String,
    pub user_id: ObjectId
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthJwtPayload {
    pub id: String,
    pub role: Role,
    pub exp: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailJwtPayload {
    pub id: String,
    pub email: String,
    pub exp: usize,
}

pub trait Expirable {
    fn exp(&self) -> usize;
    fn set_exp(&mut self, exp: usize);
}

impl Expirable for AuthJwtPayload {
    fn exp(&self) -> usize {
        self.exp
    }

    fn set_exp(&mut self, exp: usize) {
        self.exp = exp;
    }
}

impl Expirable for EmailJwtPayload {
    fn exp(&self) -> usize {
        self.exp
    }

    fn set_exp(&mut self, exp: usize) {
        self.exp = exp;
    }
}

impl From<User> for AuthJwtPayload {
    fn from(user: User) -> Self {
        Self {
            id: user.id.to_hex(),
            role: user.role.clone(),
            exp: 0
        }
    }
}

impl From<User> for EmailJwtPayload {
    fn from(user: User) -> Self {
        Self {
            id: user.id.to_hex(),
            email: user.email.clone(),
            exp: 0
        }
    }
}