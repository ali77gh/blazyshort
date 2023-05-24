use std::sync::{Arc, Mutex};
use std::collections::HashMap;

use crate::core::id_generator::seed_to_id;

#[derive(Clone)]
pub struct AppState {
    database: Arc<Mutex<HashMap<String,String>>>,
    counter: Arc<Mutex<usize>>
}

impl AppState{

    pub fn new() -> Self{
        let database = HashMap::new();
        Self { 
            database: Arc::new(Mutex::new(database.to_owned())), 
            counter: Arc::new(Mutex::new(1usize.to_owned())), 
        }
    }

    pub fn add_link(&mut self, link: String) -> Result<String, String>{
        let mut database = self.database.lock().expect("mutex was poisoned");
        let mut counter = self.counter.lock().expect("mutex was poisoned");

        let id = seed_to_id(counter.clone());
        database.insert(id.clone(), link);
        *counter += 1;

        Ok(id)
    }

    pub fn get_link_by_id(&mut self, id: String) -> Result<String, String>{
        let database = self.database.lock().expect("mutex was poisoned");
        database.get(&id).ok_or(String::from("Not Found")).cloned()
    }
}

#[cfg(test)]
mod tests {

    use super::AppState;
    #[test]
    fn save_and_load() {
        let mut state = AppState::new();
        let test_url = String::from("google.com/something_for_test");
        let id = state.add_link(test_url.clone()).unwrap();
        let url = state.get_link_by_id(id).unwrap();
        println!("{url}");
        println!("{test_url}");
        assert_eq!(url, test_url);
    }
}