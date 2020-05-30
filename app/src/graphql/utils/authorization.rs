use crate::db::mysql::DbPool;
use crate::errors::{SrvError, UnauthorizedInfo};
use crate::models::{AuthAssignmentModel as AuthAssignment, AuthItemModel as AuthItem, AuthItemChildModel as AuthItemChild, UserModel as User};
use diesel::prelude::*;
use diesel::result::Error;
use diesel::mysql::MysqlConnection;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct AuthorizationService {
    pub auth_items: Arc<Mutex<HashMap<String, AuthItem>>>,
    pub auth_relations: Arc<Mutex<HashMap<String, Vec<String>>>>,
    pub pool: Arc<DbPool>,
}

const USER_ITEM_NAME: &str = "user";
const ADMIN_ITEM_NAME: &str = "admin";

impl AuthorizationService {
    pub fn new(pool: Arc<DbPool>) -> AuthorizationService {
        AuthorizationService {
            auth_items: Arc::new(Mutex::new(HashMap::new())),
            auth_relations: Arc::new(Mutex::new(HashMap::new())),
            pool,
        }
    }

    fn auth_items_from_db(&self) -> Result<Vec<AuthItem>, Error> {
        use crate::schema::auth_items::dsl::*;
        let conn: &MysqlConnection = &self.pool.get().unwrap();
        auth_items.load(conn)
    }

    fn auth_children_from_db(&self) -> Result<Vec<AuthItemChild>, Error> {
        use crate::schema::auth_item_children::dsl::*;
        let conn: &MysqlConnection = &self.pool.get().unwrap();
        auth_item_children.load(conn)
    }

    pub fn init(&mut self) -> Result<(), Error> {
        let items = self.auth_items_from_db()?;
        let item_children = self.auth_children_from_db()?;
        {
            let mut auth_items = self.auth_items.lock().unwrap();
            for item in items {
                auth_items.insert(item.name.clone(), item.into());
            }
        }
        {
            let mut auth_relations = self.auth_relations.lock().unwrap();
            for item_child in item_children {
                match auth_relations.entry(item_child.parent) {
                    Entry::Occupied(o) => {
                        o.into_mut().push(item_child.child);
                    }
                    Entry::Vacant(v) => {
                        v.insert(vec![item_child.child]);
                    }
                };
            }
        }
        Ok(())
    }

    pub fn is_role(&self, assignments: &Option<Vec<AuthAssignment>>, role: &str) -> bool {
        match assignments {
            Some(assignments) => assignments
                .iter()
                .find(|&assignment| &assignment.item_name == role)
                .is_some(),
            None => false,
        }
    }

    pub fn is_admin(&self, assignments: &Option<Vec<AuthAssignment>>) -> bool {
        self.is_role(assignments, ADMIN_ITEM_NAME)
    }

    pub fn is_user(&self, assignments: &Option<Vec<AuthAssignment>>) -> bool {
        self.is_role(assignments, USER_ITEM_NAME)
    }

    pub fn is_authorized(&self, assignments: &Option<Vec<AuthAssignment>>, action: String) -> bool {
        match assignments {
            Some(assignments) => assignments.iter().fold(false, |acc, val| {
                acc || self._is_authorized(&val.item_name, &action)
            }),
            None => false,
        }
    }

    fn _is_authorized(&self, role: &String, action: &String) -> bool {
        let auth_items = self.auth_items.lock().unwrap();
        let auth_item = auth_items.get(role);
        return if let Some(auth_item) = auth_item {
            if auth_items.contains_key(action) {
                self.verify_in_relations(&auth_item.name, &action, &mut HashMap::new())
            } else {
                true
            }
        } else {
            false
        };
    }

    fn verify_in_relations(
        &self,
        auth_item: &String,
        action: &String,
        already_visited_actions: &mut HashMap<String, ()>,
    ) -> bool {
        let auth_relations = self.auth_relations.lock().unwrap();
        match already_visited_actions.entry(auth_item.clone()) {
            Entry::Occupied(_) => {
                return false;
            }
            Entry::Vacant(v) => v.insert(()),
        };
        match auth_relations.get(auth_item) {
            Some(relations) => match relations.iter().find(|&val| action.eq(val)) {
                Some(_) => true,
                None => relations.iter().fold(false, |acc, val| {
                    self.verify_in_relations(val, action, already_visited_actions) || acc
                }),
            },
            None => false,
        }
    }
}

pub fn assert_user(user: &Option<User>) -> Result<&User, SrvError> {
    match user {
        Some(user) => Ok(&user),
        None => Err(SrvError::Unauthorized(UnauthorizedInfo {
            data: String::from("You are not authenticated!"),
        })),
    }
}
