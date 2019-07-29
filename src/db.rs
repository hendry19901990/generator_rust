
pub fn store(conn: &mut mysql::Conn, wif: String, addr: String){
	conn.prep_exec("INSERT INTO address_rust (wif, addr) VALUES (?, ?)", (&wif, &addr)).unwrap();
}

