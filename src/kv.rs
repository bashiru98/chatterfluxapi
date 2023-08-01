use rocksdb::DB;
use std::sync::Arc;
pub trait KVStore {
    fn init(file_path: &str) -> Self;
    fn save(&self, k: &str, v: &str) -> bool;
    fn find_unique(&self, k: &str) -> Option<String>;
    fn delete(&self, k: &str) -> bool;
    fn find(&self) -> Vec<String>;
    fn save_many(&self, k: &str, v: &str) -> bool;
}
#[derive(Clone)]
pub struct RocksDB {
    db: Arc<DB>,
}
impl KVStore for RocksDB {
    fn init(file_path: &str) -> Self {
        RocksDB {
            db: Arc::new(DB::open_default(file_path).unwrap()),
        }
    }
    fn save(&self, k: &str, v: &str) -> bool {
        self.db.put(k.as_bytes(), v.as_bytes()).is_ok()
    }
    fn find_unique(&self, k: &str) -> Option<String> {
        match self.db.get(k.as_bytes()) {
            Ok(Some(v)) => {
                let result = String::from_utf8(v).unwrap();
                // println!("Finding '{}' returns '{}'", k, result);
                Some(result)
            }
            Ok(None) => {
                // println!("Finding '{}' returns None", k);
                None
            }
            Err(e) => {
                println!("Error retrieving value for {}: {}", k, e);
                None
            }
        }
    }

    fn save_many(&self, k: &str, v: &str) -> bool {
        // // convert list to string
        // let new_list = serde_json::to_string(&list).unwrap();

        // get list from db
        let list_from_db = self.find_unique(k);

        match list_from_db { 
            Some(list) => {
                // convert list to vector
                let mut list: Vec<String> = serde_json::from_str(&list).unwrap_or(Vec::new());
                // add new item to vector
                list.push(v.to_string());
                // convert vector to string
                let new_list = serde_json::to_string(&list).unwrap();
                // save to db
                self.db.put(k.as_bytes(), new_list.as_bytes()).is_ok()
            },
            None => {
                // convert list to vector
                let mut list: Vec<String> = Vec::new();
                // add new item to vector
                list.push(v.to_string());
                // convert vector to string
                let new_list = serde_json::to_string(&list).unwrap();
                // save to db
                self.db.put(k.as_bytes(), new_list.as_bytes()).is_ok()
            }
        }
    }

    fn find(&self) -> Vec<String> {
        let mut results: Vec<String> = Vec::new();
        for data in self.db.iterator(rocksdb::IteratorMode::Start) {
            let (_, value) = data.unwrap();
            let result = String::from_utf8(value.to_vec()).unwrap();
            results.push(result);
        }
        results
    }

    fn delete(&self, k: &str) -> bool {
        self.db.delete(k.as_bytes()).is_ok()
    }
}
