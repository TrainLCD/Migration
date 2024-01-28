use migration::{generator::generate_sql, migration::insert_data};

fn main() {
    let generated_sql_path = generate_sql().unwrap();
    insert_data(generated_sql_path).unwrap();
}
