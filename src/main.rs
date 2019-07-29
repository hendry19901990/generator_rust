#![allow(unused)]
#![feature(proc_macro_hygiene, decl_macro)]

extern crate bitcoin;
extern crate secp256k1;
extern crate num_bigint;
extern crate mysql;
extern crate reqwest;

#[macro_use] extern crate serde_derive;

use secp256k1::Secp256k1;
use bitcoin::util::address::Address;


pub mod db;
pub mod util;

use crate::db::*;
use crate::util::*;

/*

    CREATE TABLE `wallet`.`address_rust` ( 
      `id` INT NOT NULL AUTO_INCREMENT , 
      `wif` VARCHAR(200) NOT NULL , 
      `addr` VARCHAR(200) NOT NULL , 
      PRIMARY KEY (`id`)
    ) ENGINE = InnoDB;

*/


fn main() {
	
    let secp = Secp256k1::new();
    let pool = mysql::Pool::new("mysql://test:test@/wallet").unwrap();

    let _str = "56539106072908298546665520023773392506479484700019806659891398441363832760".to_owned();
	let mut big_int = str_to_big(_str);

    let mut len = 249;
    while len > 244 { 
        println!("{:}", big_int.to_string());
		if let Some(pri_comp) =  private_from_big(&big_int, true) {
			 let pub_key = Address::p2pkh(&pri_comp.public_key(&secp), pri_comp.network);
			 if has_balance(pub_key.to_string()) {
			 	store(&mut pool.get_conn().unwrap().unwrap(), pri_comp.to_wif(), pub_key.to_string());
			    println!("priv: {:}, pub_key: {:}", pri_comp.to_wif(), pub_key);	 
			 }
			 let pub_key = Address::p2shwpkh(&pri_comp.public_key(&secp), pri_comp.network);
			 if has_balance(pub_key.to_string()) {
			 	store(&mut pool.get_conn().unwrap().unwrap(), pri_comp.to_wif(), pub_key.to_string());
			    println!("priv: {:}, pub_key: {:}", pri_comp.to_wif(), pub_key);	 
			 }	
			 
		};

		if let Some(pri_uncomp) =  private_from_big(&big_int, false) {
			let pub_key = Address::p2pkh(&pri_uncomp.public_key(&secp), pri_uncomp.network);
			if has_balance(pub_key.to_string()) {
				store(&mut pool.get_conn().unwrap().unwrap(), pri_uncomp.to_wif(), pub_key.to_string());
				println!("priv: {:}, pub_key: {:}", pri_uncomp.to_wif(), pub_key);
			}
			let pub_key = Address::p2shwpkh(&pri_uncomp.public_key(&secp), pri_uncomp.network);
			if has_balance(pub_key.to_string()) {
				store(&mut pool.get_conn().unwrap().unwrap(), pri_uncomp.to_wif(), pub_key.to_string());
				println!("priv: {:}, pub_key: {:}", pri_uncomp.to_wif(), pub_key);
			}
			
		};

		big_int = big_int - 1u64;
		len     = big_int.bits();
	}

}

