use diesel::Insertable;
use shoe_store::schema::*;

#[derive(Insertable, Debug)]
#[table_name="products"]
pub struct NewProduct {
    pub name: String,
    pub cost: f64,
    pub active: bool,
}