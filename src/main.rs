use validator_rs::Validate;

#[allow(dead_code)]
#[derive(Validate)]
struct User {
    #[validate(min_length = 1, max_length = 50, does_not_contain = "hello")]
    name: String,

    #[validate(email)]
    email: String,

    #[validate(min = 10)]
    age: u32,

    #[validate(min = 150.0)]
    height: f32,

    address: Option<String>,

    #[validate(ipv4)]
    ip: String
}

impl User {
    fn new() -> Self {
        Self {
            name: "hakuufjhellaldsjflajsldfjalsd".to_string(),
            address: Some("fjalskdjflasjd".to_string()),
            email: "a@b.com".to_string(),
            age: 11,
            height: 170.0,
            ip: "10.0.0.1".to_string()
        }
    }
}

#[allow(dead_code)]
// #[derive(Validate)]
struct Company {
    name: String,
    total_employees: u32,
}

impl Company {
    fn new() -> Self {
        Self {
            name: "xyz".to_string(),
            total_employees: 14,
        }
    }
}

fn main() {
    let a = User::new();

    match a.validate() {
        Ok(()) => {}
        Err(err) => {
            println!("{:#?}", err);
        }
    }

    let _company = Company::new();
}
