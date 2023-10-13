use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Student {
    pub student_number: i32,
    pub ldap_dn: String,
    pub in_group: bool,
}