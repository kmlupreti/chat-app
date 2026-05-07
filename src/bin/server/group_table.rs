use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::group::Group;

pub struct GroupTable(Mutex<HashMap<Arc<String>, Arc<Group>>>);
impl GroupTable {
    pub fn new() -> Self {
        GroupTable(Mutex::new(HashMap::new()))
    }

    pub fn get(&self, group_name: &String) -> Option<Arc<Group>> {
        self.0.lock().unwrap().get(group_name).cloned()
    }

    pub fn get_or_create(&self, group_name: Arc<String>) -> Arc<Group> {
        self.0
            .lock()
            .unwrap()
            .entry(group_name.clone())
            .or_insert_with(|| Arc::new(Group::new(group_name)))
            .clone()
    }
}
