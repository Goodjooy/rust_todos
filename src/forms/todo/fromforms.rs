use rocket::FromForm;

#[derive(FromForm)]
pub struct PageRange {
    pub psize: u32,
    pub page: u32,
}
#[derive(FromForm)]
pub struct TagFilter<'s> {
    pub tgkw: Vec<&'s str>,
}
#[derive(FromForm)]
pub struct TodoFilter<'s> {
    pub tkw: &'s str,
}
