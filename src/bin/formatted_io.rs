pub fn main() {
    print!("={:width$}=\n", "a", width=5);
    print!("{:^8}", "a");
    print!("{:>10} {:>10}", "a", "b" );    


    print!("\n\n\n");
    let column_headers = vec!("experiment_key", "name", "variant_key", "name",  "percent count");
    for c in &column_headers {
        print!("{:^width$} ", c, width = c.len());
    }
    print!("\n");
    for c in &column_headers {
        print!("{x:-<0$} ", c.len(), x = "-");

    }

}


