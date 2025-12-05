use wasm_bindgen::prelude::* ;
use web_sys::console::log_1 as log ;
use wee_alloc::WeeAlloc ;

#[global_allocator]
static ALLOC : WeeAlloc = WeeAlloc::INIT ;

#[wasm_bindgen]
pub fn load_balance(balance_file: &str) {
    
    //balance_file.split("\n").into_iter().for_each(|t|log(&t.replace(",", ".").into())) ;
    balance_file.split("\n").into_iter().for_each(|t:&str|poe_no_console(&t.replace(",","."))) ;
}

fn poe_no_console( linha: &String) {
    let broaken: Vec<&str> = linha.split(";").collect() ;
    let broaken_line = format!("{{\"{}\":\"{}\",\"{}\":\"{}\",\"{}\":{},\"{}\":\"{}\"}}",
        "date", broaken[0],
        "description", broaken[1],
        "amount", broaken[2],
        "catogory", "UNCATEGORIZED"
    ) ;
    println!("{}", &broaken_line) ;
    log(&broaken_line.into()) ;
}