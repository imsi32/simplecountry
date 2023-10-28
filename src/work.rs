//! The [`Work`] struct and its associated `impl`s.

use time::Date;
use crate::thing::Thing;
use crate::person::Person;

#[derive(Debug, PartialEq)]
pub struct Work<'a> {
    name: String,
    description: String,
    interval: (Date, Date),
    tags: &'a [String],
    details: &'a [String],
    persons: &'a [Person<'a>],
}

impl<'a> Default for Work<'a>{
    fn default() -> Self {
        Self {
            name: String::new(),
            description: String::new(),
            interval: (Date::MIN, Date::MAX),
            tags: &[],
            details: &[],
            persons: &[],
        }
    }
}

impl<'a> Thing<'a> for Work<'a>{
    fn get_name(&self) -> String { self.name.clone() }
    fn get_description(&self) -> String { self.description.clone() }
    fn get_interval(&self) -> (Date, Date) { self.interval }
    fn get_tags(&self) -> &'a [String] { self.tags }
    fn get_details(self) -> &'a [String] { self.details }
}

impl<'a> Work<'a> {
    pub fn get_persons(self) -> &'a [Person<'a>] { self.persons }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn work_default() {
        let work = Work::default();
        assert_eq!(work, Work{
            name: String::new(),
            description: String::new(),
            interval: (Date::MIN, Date::MAX),
            tags: &[],
            details: &[],
            persons: &[],
        })
    }
}
