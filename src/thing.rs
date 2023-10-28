//! The [`Thing`] trait and its `fn`s.
use time::Date;

pub trait Thing<'a> {
    /// For returning [`Thing`]'s name.
    fn get_name(&self) -> String;

    /// For returning [`Thing`]'s description.
    fn get_description(&self) -> String;

    /// For returning [`Thing`]'s time interval.
    fn get_interval(&self) -> (Date, Date);

    /// For returning [`Thing`]'s associated tags.
    fn get_tags(&self) -> &'a [String];

    /// For returning [`Thing`]'s details.
    fn get_details(self) -> &'a [String];

    /// For finding tag inside [`Thing`]'s associated tags.
    fn is_in_tag(&self, tag: String) -> bool {
        self.get_tags().contains(&tag)
    }
}
