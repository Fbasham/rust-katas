mod preloaded;
use preloaded::{Coffee, Milk, Sugar};

struct CoffeeBuilder {
    sort: String,
    milk: Vec<Milk>,
    sugar: Vec<Sugar>,
}

impl CoffeeBuilder {
    fn new() -> CoffeeBuilder {
        CoffeeBuilder {
            sort: String::from(""),
            milk: vec![],
            sugar: vec![]
        }
    }

    fn set_black_coffee(mut self) -> CoffeeBuilder {
        self.sort = String::from("Black");
        self
    }

    fn set_cubano_coffee(mut self) -> CoffeeBuilder {
        self.sort = String::from("Cubano");
        self.sugar.push(Sugar{sort:"Brown".to_string()});
        self
    }

    fn set_antoccino_coffee(mut self) -> CoffeeBuilder {
        self.sort = String::from("Americano");
        self.milk.push(Milk{fat:0.5});
        self
    }

    fn with_milk(mut self, fat: f32) -> CoffeeBuilder {
        self.milk.push(Milk{fat:fat});
        self
    }

    fn with_sugar(mut self, sort: String) -> CoffeeBuilder {
        self.sugar.push(Sugar{sort:sort.to_string()});
        self
    }

    fn build(self) -> Coffee {
        Coffee{sort:self.sort,milk:self.milk,sugar:self.sugar}
    }
}