#[macro_use]
extern crate diesel;

use anyhow::Result;
use diesel::sqlite::SqliteConnection;
use diesel::ExpressionMethods;
use ::shoe_store::models::*;
use diesel::Connection;
use diesel::RunQueryDsl;
use diesel::query_dsl::QueryDsl;

no_arg_sql_function!(last_insert_rowid, diesel::sql_types::Integer);

fn create_product(new_product: NewCompleteProduct, conn: &SqliteConnection) -> Result<i32>  {
    use ::shoe_store::schema::products::dsl::products;
    use ::shoe_store::schema::variants::dsl::*;
    use ::shoe_store::schema::products_variants::dsl::*;

    conn.transaction(|| {
        diesel::insert_into(products)
            .values(new_product.product)
            .execute(conn)?;

        let last_product_id: i32 = diesel::select(last_insert_rowid).first(conn)?;

        for new_variant in new_product.variants {
            let variants_result =
                variants
                    .filter(name.eq(&new_variant.variant.name))
                    .limit(1)
                    .load::<Variant>(conn)?;

            let last_variant_id: i32 =
                match variants_result.first() {
                    Some(variant) => variant.id,
                    None => {
                        diesel::insert_into(variants)
                            .values(name.eq(&new_variant.variant.name))
                            .execute(conn)?;

                        diesel::select(last_insert_rowid).first(conn)?
                    }
                };

            for new_value in new_variant.values {
                diesel::insert_into(products_variants)
                    .values(
                        (
                            product_id.eq(last_product_id), 
                            variant_id.eq(last_variant_id),
                            value.eq(new_value), 
                        )
                    )
                    .execute(conn)?;
            }
        }
        Ok(last_product_id)
    })
}