
use mysql::*;
use std::collections::HashMap;
use core::hash::BuildHasher;


fn sample_column_sizes<H: BuildHasher>(columns: &Vec<Column>, column_indexes: HashMap<String, usize, H>, rows: &Vec<Row>)  {
    let mut sizes: HashMap<String, u8> = HashMap::new();

    for row in rows {
        let mut header_line = String::new();
        let mut header_underline = String::new();
        for (column_name, index) in column_indexes.iter() {
            
            let value = row.as_ref(index.clone()).unwrap().into_str();
            //println!("row: {:?}, {}", column_name, row.as_ref(index.clone()).unwrap().into_str());
            let x = format!("{}", column_name);
            header_line.push_str(&x);
            let y = format!("{x:-<0$} ", value.len(), x = "-");
            header_underline.push_str(&y);

            print!("{x:-<0$} ", value.len(), x = "-");
           
        }
        print!("\n");
        print!("{}\n", header_line);
        print!("{}\n", header_underline);
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

        let columns = result.columns_ref().to_vec();
        let column_indexes = result.column_indexes();
        let rows = result.map(|row| row.unwrap() ).collect::<Vec<_>>(); 

        sample_column_sizes(&columns, column_indexes, &rows);

 
        // println!("column_indexes: {:?}", result.column_indexes());
        // println!("columns: {:?}", result.columns_ref());

        // for column in result.columns_ref() {
        //     println!("column {:?}", column);
        // }
        
        // for row in result {
        //     let mut unwrapped = row.unwrap();

        //     println!("row len: {}", unwrapped.len());
        //     let row_value_0: String  = unwrapped.get(0).unwrap();
        //     println!("row[0]: {:?}", row_value_0);
        //     let (catalog_name, schema_name, default_character_set_name): (String, String, String) = from_row(unwrapped);
        //     println!("schema_name {}, character set {}", schema_name, default_character_set_name);
        // }
        
    });
}
