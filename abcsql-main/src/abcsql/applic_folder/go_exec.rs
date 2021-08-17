// use gluesql::ast::Statement;
#[cfg(feature = "sled-storage")]
use gluesql::{parse, Glue, Payload::*};

// use sqlparser::ast::OrderByExpr;

use std::rc::Rc;

use crate::applic_folder::compileargs::*;
use crate::applic_folder::get_storage::get_storage;
use crate::applic_folder::orderby::*;
use crate::applic_folder::show_select::*;
/***************************************************/
pub fn go_exec(how: Rc<HowToOpenStorage>, one_query: String, seen: &Seen) {
    let storage = get_storage(how);
    let mut glue = Glue::new(storage);
    let statments1 = parse(&one_query);
    match statments1 {
        Ok(statments) => {
            for query in statments {
                // match statement {
                //     Ok(query) =>{
                match &glue.execute(&query.to_string()) {
                    Ok(Select { labels, rows }) => {
                        let mut order_by: Vec<gluesql::parser::ast::OrderByExpr> = Vec::new();
                        match query {
                            // Query(xxx) => {
                            //     match &xxx {
                            gluesql::parser::ast::Statement::Query(yyy) => {
                                order_by = yyy.order_by.clone();
                            }
                            _ => {
                                eprintln!("???{}", query);
                            } //     };
                              // }
                        }
                        let rows = orderby(labels.to_vec(), rows.to_vec(), &order_by);
                        show_select(
                            labels.to_vec(),
                            rows.to_vec(),
                            if seen.printsqlstm {
                                Some(&one_query)
                            } else {
                                None
                            },
                        )
                    }
                    Ok(Insert(n)) => eprintln!("{} rows inserted", n),
                    Ok(Delete(n)) => eprintln!("{} rows deleted", n),
                    Ok(Update(n)) => eprintln!("{} rows update", n),
                    Ok(Create) => eprintln!("Table created"),
                    Ok(DropTable) => eprintln!("Table dropped"),
                    Err(e) => eprintln!("Error: {}", e),
                    _ => eprintln!("Not yet fom glue.execute"),
                }
                // },
                // Err(e)=>eprintln!("Error: {}", e),
                // }
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}
