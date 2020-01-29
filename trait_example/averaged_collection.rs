pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            },
            None => None,
        }
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}

pub trait Average {
    fn average(&self) -> f64;
}

impl Average for AveragedCollection {
    fn average(&self) -> f64 {
        self.average
    }
}

fn main() {
    let mut a = AveragedCollection { list: Vec::new(), average: 0. };
    a.add(42);
    a.add(21);
    a.add(10);
    println!("{}", a.average());
}
