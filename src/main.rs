mod table;

use crate::table::*;

fn main() {
    let my_data = vec![
        vec!["Nome".to_string(), "Idade".to_string()],
        vec!["Lucas".to_string(), "19".to_string()],
        vec!["Maria".to_string(), "30".to_string()],
        vec!["Dracula".to_string(), "5005".to_string()],
    ];


    let table = Table::new(my_data.clone(), "lines").unwrap();
    // println!("{:?}", table.style.border);
    table.plot();

    
}
