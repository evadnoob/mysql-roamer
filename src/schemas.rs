
use mysql::*;
use std::collections::HashMap;
use core::hash::BuildHasher;


fn sample_column_sizes<H: BuildHasher>(columns: &Vec<Column>, column_indexes: HashMap<String, usize, H>, rows: &Vec<Row>)  {
    let mut sizes: HashMap<String, u8> = HashMap::new();

    for row in rows {
        for (column_name, index) in column_indexes.iter() {
            //println!("row: {:?}, {}", column_name, row.as_ref(index.clone()).unwrap().into_str());
            let value = row.as_ref(index.clone()).unwrap().into_str();
            print!("{x:-<0$} ", value.len(), x = "-");

           
        }
        print!("\n");
    }
}



///
/// dump information about all schemas in all 
/// 
pub fn all(url: &str) {
    let opts = Opts::from_url(url);
    println!("opts {:?}", opts);
    let  pool = Pool::new(opts.unwrap()).unwrap();
    
    pool.prep_exec("select catalog_name, schema_name, default_character_set_name from information_schema.schemata", ()).map(|result| {

        println!("column_indexes: {:?}", result.column_indexes());
        println!("columns: {:?}", result.columns_ref());

        for column in result.columns_ref() {
            println!("column {:?}", column);
        }
        
        for row in result {
            let mut unwrapped = row.unwrap();

            println!("row len: {}", unwrapped.len());
            let row_value_0: String  = unwrapped.get(0).unwrap();
            println!("row[0]: {:?}", row_value_0);
            let (catalog_name, schema_name, default_character_set_name): (String, String, String) = from_row(unwrapped);
            println!("schema_name {}, character set {}", schema_name, default_character_set_name);
        }
        
    });
}