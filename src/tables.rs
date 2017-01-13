
use mysql::*;


///
/// dump information about all tables in all schemas
/// 
pub fn all(url: &str) {
    let opts = Opts::from_url(url);
    println!("opts {:?}", opts);
    let  pool = Pool::new(opts.unwrap()).unwrap();
    
    pool.prep_exec("select table_name, table_schema from information_schema.tables", ()).map(|result| {

        println!("column_indexes: {:?}", result.column_indexes());
        println!("columns: {:?}", result.columns_ref());
        
        for row in result {
            let mut unwrapped = row.unwrap();

            println!("row len: {}", unwrapped.len());
            let row_value_0: String  = unwrapped.get(0).unwrap();
            println!("row[0]: {:?}", row_value_0);
            let (table_name, table_schema): (String, String) = from_row(unwrapped);
            println!("table_name {}.{}", table_schema, table_name);
        }
        
    });
}
