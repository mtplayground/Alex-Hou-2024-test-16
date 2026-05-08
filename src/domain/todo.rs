use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Todo {
    pub id: i64,
    pub title: String,
    pub completed: bool,
    pub position: i64,
}

#[cfg(feature = "ssr")]
impl From<crate::db::todos::TodoRow> for Todo {
    fn from(value: crate::db::todos::TodoRow) -> Self {
        Self {
            id: value.id,
            title: value.title,
            completed: value.completed,
            position: value.position,
        }
    }
}
