
const INST_EMAIL: &str = "@institute.org";

pub enum EntityType {
    Admin,
    Prof{dept: String}, 
    Student(String), //For a student, we will need their class
}


pub struct Entity {
   first_name: String,
   last_name: String,
   entity_type: EntityType, 
}

impl Entity {
    /// Builds email in the form: first_name.last_name.suffix@institute.org
    /// suffix = admin | dept | class
    pub fn get_email(&self) -> String {
        let mut email = String::from(&self.first_name);
        email.push_str(".");
        email.push_str(&self.last_name);
        email.push_str(".");

        match &self.entity_type {
            EntityType::Admin => email.push_str("admin"),
            EntityType::Prof { dept } => email.push_str(&dept),
            EntityType::Student(class) => email.push_str(&class),
        };
        
        email.push_str(INST_EMAIL);

        email
    }
}






#[cfg(test)]
mod tests {
    use super::*;

   

    #[test]
    fn compue_email() {
        let prof = Entity {
            first_name: String::from("Foo"),
            last_name: String::from("Bar"),
            entity_type: EntityType::Prof{dept: String::from("CS")},
        };
        let computed_email = prof.get_email();
        assert_eq!(computed_email, String::from("Foo.Bar.CS@institute.org"))
    }

}