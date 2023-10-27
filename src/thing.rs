use time::Date;

pub trait Thing<'a> {
    fn get_name(&self) -> String;
    fn get_description(&self) -> String;
    fn get_interval(&self) -> (Date, Date);
    fn get_tags(&self) -> &'a [String];
    fn is_in_tag(&self, tag: String) -> bool {
        self.get_tags().contains(&tag)
    }
}
