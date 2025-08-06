use std::collections::HashMap;

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

struct InMemoryRepository {
    db: HashMap<u16, User>,
    next_id: u16,
}

impl InMemoryRepository{
    fn new()->Self {
        Self {
            db: HashMap::new(),
            next_id: 1,
        }
    }
}
impl UserRepository for InMemoryRepository {
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

fn main() {
    let mut mem_repo = InMemoryRepository::new();
    mem_repo.save(User{id: 0, name: "Jamal".into(), email: "jamal@gmail.com".into()}).unwrap();
    mem_repo.save(User{id: 0, name: "Kano".into(), email: "kano@gmail.com".into()}).unwrap();
    mem_repo.save(User{id: 0, name: "Al".into(), email: "al@gmail.com".into()}).unwrap();
    mem_repo.save(User{id: 0, name: "Oppa".into(), email: "oppa@gmail.com".into()}).unwrap();
    println!("{:#?}", mem_repo.find_by_email("al@gmail.com".into()).unwrap());
    println!("{:#?}", mem_repo.find_by_id(2).unwrap());
}
