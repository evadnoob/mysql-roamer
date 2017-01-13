extern crate mysql;
extern crate core;

use mysql::*;
use std::collections::HashMap;
use core::hash::BuildHasher;

//
// scan some rows, find the length of the rows
//
fn sample_column_sizes<H: BuildHasher>(columns: &Vec<Column>, column_indexes: HashMap<String, usize, H>, rows: &Vec<Row>)  {
    let mut sizes: HashMap<String, u8> = HashMap::new();

    for row in rows {
        for (column_name, index) in column_indexes.iter() {
            println!("row: {:?}, {}", column_name, row.as_ref(index.clone()).unwrap().into_str());
        }
    }
}

//
//
//
pub fn main() {
    let url = "mysql://david:david@172.17.0.2:3306/test";
    let opts = Opts::from_url(url);
    println!("opts {:?}", opts);
    let  pool = Pool::new(opts.unwrap()).unwrap();
    
    pool.prep_exec("select catalog_name, schema_name, default_character_set_name from information_schema.schemata", ()).map(|result| {

        println!("column_indexes: {:?}", result.column_indexes());
        println!("columns: {:?}", result.columns_ref());
        
        let columns = result.columns_ref().to_vec();
        let column_indexes = result.column_indexes();
        let rows = result.map(|row| row.unwrap() ).collect::<Vec<_>>(); 

        sample_column_sizes(&columns, column_indexes, &rows);

        for row in rows {
        }
        
        
    });
    
}
