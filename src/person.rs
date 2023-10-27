use time::Date;
use crate::thing::Thing;
use crate::{
    event::Event,
    work::Work,
};

#[derive(Debug, PartialEq)]
pub struct Person<'a> {
    name: String,
    description: String,
    interval: (Date, Date),
    tags: &'a [String],
    details: &'a [String],
    events: &'a [Event<'a>],
    works: &'a [Work<'a>],
}

impl<'a> Default for Person<'a>{
    fn default() -> Self {
        Self {
            name: String::new(),
            description: String::new(),
            interval: (Date::MIN, Date::MAX),
            tags: &[],
            details: &[],
            events: &[],
            works: &[],
        }
    }
}

impl<'a> Thing<'a> for Person<'a>{
    fn get_name(&self) -> String { self.name.clone() }
    fn get_description(&self) -> String { self.description.clone() }
    fn get_interval(&self) -> (Date, Date) { self.interval }
    fn get_tags(&self) -> &'a [String] { self.tags }
}

impl<'a> Person<'a> {
    pub fn get_details(self) -> &'a [String] { self.details }
    pub fn get_events(self) -> &'a [Event<'a>] { self.events }
    pub fn get_works(self) -> &'a [Work<'a>] { self.works }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn person_default() {
        let person = Person::default();
        assert_eq!(person, Person{
            name: String::new(),
            description: String::new(),
            interval: (Date::MIN, Date::MAX),
            tags: &[],
            details: &[],
            events: &[],
            works: &[],
        })
    }
}
