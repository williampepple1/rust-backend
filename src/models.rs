use diesel::{ Insertable, Identifiable, Queryable};
use serde::{Serialize, Deserialize};
use shoe_store::schema::variants;

#[derive(Identifiable, Queryable, Debug, Serialize, Deserialize)]
#[table_name = "variants"]
pub struct Variant {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable, Debug, Clone)]
#[table_name="variants"]
pub struct NewVariant {
    pub name: String,
}

use shoe_store::schema::products_variants;

#[derive(Insertable, Debug)]
#[table_name="products_variants"]
pub struct NewProductVariant {
    pub product_id: i32,
    pub variant_id: i32,
    pub value: Option<String>
}

#[derive(Clone)]
pub struct NewVariantValue {
    pub variant: NewVariant,
    pub values: Vec<Option<String>>
}

pub struct NewCompleteProduct {
    pub product: NewProduct,
    pub variants: Vec<NewVariantValue>
}