use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Arc;

#[derive(Debug, Clone)]
struct User {
    id: u16,
    name: String,
    email: String,
}

trait UserRepository {
    fn find_by_id(&self, id: u16) -> Option<User>;
    fn save(&mut self, user: User) -> Result<(), String>;
    fn find_by_email(&self, email: String) -> Option<User>;
}

struct InMemoryUserRepository {
    db: HashMap<u16, User>,
    next_id: u16,
}

impl InMemoryUserRepository {
    fn new() -> Self {
        Self {
            db: HashMap::new(),
            next_id: 1,
        }
    }
}
impl UserRepository for InMemoryUserRepository {
    fn find_by_id(&self, id: u16) -> Option<User> {
        self.db.get(&id).cloned()
    }

    fn find_by_email(&self, email: String) -> Option<User> {
        self.db.values().find(|&user| user.email == email).cloned()
    }

    fn save(&mut self, mut user: User) -> Result<(), String> {
        if user.id == 0 {
            user.id = self.next_id;
            self.next_id += 1;
        }
        self.db.insert(self.next_id, user);
        Ok(())
    }
}

struct DatabaseUserRepository {
    connection_string: String,
}

impl DatabaseUserRepository {
    fn new(connection_string: String) -> Self {
        Self { connection_string }
    }
}

impl UserRepository for DatabaseUserRepository {
    fn find_by_id(&self, id: u16) -> Option<User> {
        Some(User {
            id,
            email: "Anone@gmail.com".into(),
            name: "Anone".into(),
        })
    }
    fn find_by_email(&self, email: String) -> Option<User> {
        Some(User {
            id: 0,
            email,
            name: "Anone".into(),
        })
    }
    fn save(&mut self, _user: User) -> Result<(), String> {
        println!("Saving stuff!");
        Ok(())
    }
}

trait EmailSender {
    fn send_email(&self, to: &str, msg: &str) -> Result<(), String>;
}

struct EmailService;
impl EmailService {
    fn new() -> Self {Self}
    fn send_email(&self, to: &str, msg: &str) -> Result<(), String> {
        println!("Sending mail to: {to}, \n {msg}");
        Ok(())
    }
}

struct MockEmailService {
    email_list: Rc<RefCell<Vec<(String, String)>>>,
}
impl MockEmailService {
    fn new() -> Self { Self{ email_list: Rc::new(RefCell::new(Vec::new()))}}
    fn send_email(&self, to: &str, msg: &str) -> Result<(), String> {
        println!("saving mail to archive!");
        self.email_list.borrow_mut().push((to.into(), msg.into()));
        println!("Sending mail to: {to}, \n {msg}");
        Ok(())
    }
}

struct UserManagementService<E: EmailSender>{}
fn main() {
    // let mut mem_repo = InMemoryUserRepository::new();
    // let mut mem_repo = DatabaseUserRepository::new("postgres://network-protocol.com".into());
    // mem_repo
    //     .save(User {
    //         id: 0,
    //         name: "Jamal".into(),
    //         email: "jamal@gmail.com".into(),
    //     })
    //     .unwrap();
    // mem_repo
    //     .save(User {
    //         id: 0,
    //         name: "Kano".into(),
    //         email: "kano@gmail.com".into(),
    //     })
    //     .unwrap();
    // mem_repo
    //     .save(User {
    //         id: 0,
    //         name: "Al".into(),
    //         email: "al@gmail.com".into(),
    //     })
    //     .unwrap();
    // mem_repo
    //     .save(User {
    //         id: 0,
    //         name: "Oppa".into(),
    //         email: "oppa@gmail.com".into(),
    //     })
    //     .unwrap();
    // println!(
    //     "{:#?}",
    //     mem_repo.find_by_email("al@gmail.com".into()).unwrap()
    // );
    // println!("{:#?}", mem_repo.find_by_id(2).unwrap());

    let email_service = EmailService::new();
    let
}
