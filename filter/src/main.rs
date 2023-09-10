struct Customer {
    id: u32,
    name: String,
    age: u32,
}

impl Customer {
    fn new(id: u32, name: &str, age: u32) -> Self {
        Customer {
            id,
            name: name.to_string(),
            age,
        }
    }
}

//Define the FilterCondition struct with the desired type for filtering
enum FilterCondition {
    Id(u32),
    Age(u32, u32),
    Name(String),
}

trait Filter<T> {
    fn is_match(&self, item: &T) -> bool;
}
//Implement the is_match method on the FilterCondition struct
impl Filter<Customer> for FilterCondition {
    fn is_match(&self, customer: &Customer) -> bool {
        match self {
            FilterCondition::Id(id) => customer.id == *id,
            FilterCondition::Age(min, max) => customer.age >= *min && customer.age <= *max,
            FilterCondition::Name(name) => customer.name == *name,
        }
    }
}

fn custom_filter<'a, T: Filter<Customer>>(
    customers: &'a [Customer],
    filter_condition: T,
) -> Vec<&'a Customer> {
    customers
        .iter()
        .filter(|customer| filter_condition.is_match(customer))
        .collect()
}

fn main() {
    let customers = vec![
        Customer::new(1, "Alice", 21),
        Customer::new(2, "From", 18),
        Customer::new(3, "Wonderland", 74),
    ];

    let filter_condition = FilterCondition::Age(18, 30);

    let filtered_customers: Vec<&Customer> = custom_filter(&customers,filter_condition);

    println!("Filtered Customers:");
    for customer in filtered_customers {
        println!(
            "ID: {}, Name: {}, Age: {}",
            customer.id, customer.name, customer.age
        );
    }
}

