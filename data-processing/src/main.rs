trait DataPipeline<T> {
    fn insert(&mut self, item: T);

    fn insert_many(&mut self, items: impl Iterator<Item = T>);

    fn num_valid(&self) -> u32;

    fn filter(&mut self, predicate: fn(&T) -> bool);
}

struct Validator<T> {
    storage_vec: Vec<T>,
}

impl<T> Validator<T> {
    fn new() -> Self {
        Validator {
            storage_vec: vec![],
        }
    }
}

impl<T> DataPipeline<T> for Validator<T> {
    fn insert(&mut self, item: T) {
        self.storage_vec.push(item);
    }

    fn insert_many(&mut self, items: impl Iterator<Item = T>) {
        self.storage_vec.extend(items);
    }

    fn num_valid(&self) -> u32 {
        self.storage_vec.len() as u32
    }

    fn filter(&mut self, predicate: fn(&T) -> bool) {
        self.storage_vec.retain(predicate)
    }
}

fn main() {
    let mut vals = Validator::<i32>::new();
    vals.insert(1);
    vals.insert(10);
    vals.insert(100);
    vals.filter(|&x| x < 50);

    assert_eq!(vals.num_valid(), 2);
}
