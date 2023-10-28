//! The [`Country`] struct and its associated `impl`s.

use time::Date;
use crate::thing::Thing;
use crate::continent::Continent;
use crate::{
    event::Event,
    work::Work,
    person::Person,
};

#[derive(Debug, PartialEq)]
pub struct Country<'a> {
    name: String,
    description: String,
    interval: (Date, Date),
    tags: &'a [String],
    details: &'a [String],
    continent: &'a [Continent],
    events: &'a [Event<'a>],
    works: &'a [Work<'a>],
    persons: &'a [Person<'a>],
}

impl<'a> Default for Country<'a>{
    fn default() -> Self {
        Self {
            name: String::new(),
            description: String::new(),
            interval: (Date::MIN, Date::MAX),
            tags: &[],
            details: &[],
            continent: &[],
            events: &[],
            works: &[],
            persons: &[],
        }
    }
}

impl<'a> Thing<'a> for Country<'a>{
    fn get_name(&self) -> String { self.name.clone() }
    fn get_description(&self) -> String { self.description.clone() }
    fn get_interval(&self) -> (Date, Date) { self.interval }
    fn get_tags(&self) -> &'a [String] { self.tags }
    fn get_details(self) -> &'a [String] { self.details }
}

impl<'a> Country<'a> {
    pub fn get_continent(self) -> &'a [Continent] { self.continent }
    pub fn get_events(self) -> &'a [Event<'a>] { self.events }
    pub fn get_works(self) -> &'a [Work<'a>] { self.works }
    pub fn get_persons(self) -> &'a [Person<'a>] { self.persons }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn country_default() {
        let country = Country::default();
        assert_eq!(country, Country{
            name: String::new(),
            description: String::new(),
            interval: (Date::MIN, Date::MAX),
            tags: &[],
            details: &[],
            continent: &[],
            events: &[],
            works: &[],
            persons: &[],
        })
    }
}
