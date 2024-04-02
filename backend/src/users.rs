struct User<'a> {
    name: &'a str,
    password: &'a str,
}

const USER_LIST: [User; 3] = [
    User {
        name: "Vincent",
        password: "1234",
    },
    User {
        name: "Emma",
        password: "0000",
    },
    User {
        name: "Thibault",
        password: "2001",
    },
];

pub fn is_user(login: &str, password: &str) -> bool {
    for usr in USER_LIST {
        if usr.name == login && usr.password == password {
            return true;
        }
    }
    false
}
