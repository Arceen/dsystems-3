use aws_sdk_s3::config::endpoint::SharedEndpointResolver;
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
impl User {
    fn new(name: impl Into<String>, email: impl Into<String>) -> Self {
        Self {
            id: 0,
            name: name.into(),
            email: email.into(),
        }
    }
}

trait UserRepository {
    fn find_by_id(&self, id: u16) -> Option<User>;
    fn save(&mut self, user: User) -> Result<(), String>;
    fn find_by_email(&self, email: impl Into<String>) -> Option<User>;
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

    fn find_by_email(&self, email: impl Into<String>) -> Option<User> {
        let email = email.into();
        self.db.values().find(|&user| user.email == email).cloned()
    }

    fn save(&mut self, mut user: User) -> Result<(), String> {
        if user.id == 0 {
            user.id = self.next_id;
        }
        self.db.insert(self.next_id, user);
        self.next_id += 1;
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
    fn find_by_email(&self, email: impl Into<String>) -> Option<User> {
        Some(User {
            id: 0,
            email: email.into(),
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
    fn new() -> Self {
        Self
    }
}
impl EmailSender for EmailService {
    fn send_email(&self, to: &str, msg: &str) -> Result<(), String> {
        println!("Sending mail to: {to}, \n {msg}");
        Ok(())
    }
}
struct MockEmailService {
    email_list: Rc<RefCell<Vec<(String, String)>>>,
}
impl MockEmailService {
    fn new() -> Self {
        Self {
            email_list: Rc::new(RefCell::new(Vec::new())),
        }
    }
}

impl EmailSender for MockEmailService {
    fn send_email(&self, to: &str, msg: &str) -> Result<(), String> {
        println!("saving mail to archive!");
        self.email_list.borrow_mut().push((to.into(), msg.into()));
        println!("Sending mail to: {to}, \n {msg}");
        Ok(())
    }
}

struct UserManagementService<E: EmailSender, R: UserRepository> {
    email_sender: E,
    user_repository: R,
}

impl<E: EmailSender, R: UserRepository> UserManagementService<E, R> {
    fn new(email_sender: E, user_repository: R) -> Self {
        Self {
            email_sender,
            user_repository,
        }
    }
    fn register_user(&mut self, name: &str, email: &str) -> Result<(), String> {
        self.user_repository.save(User::new(name, email))?;
        self.email_sender
            .send_email(email, "Welcome to our school")?;
        Ok(())
    }
}

struct SharedUserService {
    user_repository: Rc<RefCell<dyn UserRepository>>,
    email_service: Rc<dyn EmailSender>,
}
impl SharedUserService {
    fn new(
        user_repository: Rc<RefCell<dyn UserRepository>>,
        email_service: Rc<dyn EmailSender>,
    ) -> Self {
        Self {
            user_repository,
            email_service,
        }
    }

    fn register_user(&mut self, name: &str, email: &str) -> Result<(), String> {
        self.user_repository
            .borrow_mut()
            .save(User::new(name, email))?;
        self.email_service
            .send_email(email, "Welcome to our school")?;
        Ok(())
    }
}

struct DIContainer {
    email_sender: Rc<dyn EmailSender>,
    user_repository: Rc<RefCell<dyn UserRepository>>,
}

impl DIContainer {
    fn new() -> Self {
        Self {
            email_sender: Rc::new(EmailService::new()),
            user_repository: Rc::new(RefCell::new(InMemoryUserRepository::new())),
        }
    }

    fn with_email_sender(mut self, sender: Rc<dyn EmailSender>) -> Self {
        self.email_sender = sender;
        self
    }
    fn with_user_repository(mut self, repo: Rc<RefCell<dyn UserRepository>>) -> Self {
        self.user_repository = repo;
        self
    }

    fn create_user_service(&self) -> SharedUserService {
        SharedUserService::new(self.user_repository.clone(), self.email_sender.clone())
    }
}

fn main() -> Result<(), String> {
    let user_repository = Rc::new(RefCell::new(InMemoryUserRepository::new()));
    let email_service = Rc::new(EmailService::new());
    let mut shared_email_management =
        SharedUserService::new(user_repository.clone(), email_service.clone());
    shared_email_management.register_user("Niloy", "niloyman01@gmail.com")?;
    println!("{:#?}", user_repository.borrow().find_by_id(1));
    shared_email_management.Ok(())
}
