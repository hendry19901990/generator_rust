
pub fn store(conn: &mut mysql::Conn, wif: String, addr: String){
    if !conn.ping() {
        conn.reset().unwrap();
    }
    
	conn.prep_exec("INSERT INTO address_rust (wif, addr) VALUES (?, ?)", (&wif, &addr)).unwrap();
}

pub fn exist_address(conn: &mut mysql::Conn, address: String) -> bool {
	use std::borrow::Borrow;

    if !conn.ping() {
        conn.reset().unwrap();
    }

    let mut result= conn.prep_exec("SELECT id FROM rich_list where address = ?", (&address, )).unwrap();

    match result.next() {
    	Some(result2) => {
    		match result2 {
    			Ok(mut result3) => {
                    let opt: Option<i64> = result3.take(0);
				    if let Some(id) = opt {
				    	println!("{:?}", id);
				     	return id != 0
				    }else {
				      	return false
				    }
    			},
    			Err(_) => return false,
    		}
    	},
    	None => return false,
    }
 }

