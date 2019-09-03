#![allow(unused)]
#![feature(proc_macro_hygiene, decl_macro)]

extern crate bitcoin;
extern crate secp256k1;
extern crate num_bigint;
extern crate mysql;
extern crate reqwest;

#[macro_use] extern crate serde_derive;

extern crate sha2;

use secp256k1::Secp256k1;
use bitcoin::util::address::Address;

pub mod db;
pub mod util;

use crate::db::*;
use crate::util::*;

use sha2::{Sha256, Digest};


/*

    CREATE TABLE `wallet`.`address_rust` ( 
      `id` INT NOT NULL AUTO_INCREMENT , 
      `wif` VARCHAR(200) NOT NULL , 
      `addr` VARCHAR(200) NOT NULL , 
      PRIMARY KEY (`id`)
    ) ENGINE = InnoDB;

*/


fn main() {
    //read_every_bit();
  // read_file();
   read_by_range();
}

fn read_file() {
	use std::fs::File;
	use std::io::BufReader; 
	use std::io::BufRead;

	let pool = mysql::Pool::new("mysql://test:test@/wallet").unwrap();

	let file = File::open("C:/Users/HENDRY/Documents/numeros_primos/facebook-firstnames.txt").unwrap();
	let mut reader = BufReader::new(file);

    loop {
        let mut number_str = String::new();

        match reader.read_line(&mut number_str) {
            Ok(n) => {
            	if n == 0 { break; }

            	let mut hasher = Sha256::new();
            	let str_c = number_str.clone();

				// write input message
				hasher.input(&number_str.into_bytes());
				let result  = &hasher.result().to_vec();

				let str_hex = hex::encode(result);
				println!("{:} => {:}", str_c, &str_hex);

			    let mut big_int = str_to_big(str_hex);
			    call(&big_int, &mut pool.get_conn().unwrap().unwrap());  
			        	
            },
            Err(e) => {  }
        }

    }

}

fn read_by_range(){
	
    let pool = mysql::Pool::new("mysql://test:test@/wallet").unwrap();

    let _str     = "45408662446006351146498425493603101118929405751231593740963758434475737113700".to_owned();
    let _str_end = "57669001306428065956053000376875938421040345304064124051023973211784186134399".to_owned();
	let mut big_int     = str_to_big(_str);
	let mut big_int_end = str_to_big(_str_end);

     
    while big_int < big_int_end { 
        println!("{:}", big_int.to_string());

        call(&big_int, &mut pool.get_conn().unwrap().unwrap());

		big_int = big_int + 1u64;
	}
}

fn read_every_bit(){
	
    let pool = mysql::Pool::new("mysql://test:test@/wallet").unwrap();

    let _str = "5653910607290829854666552002377339250647948470001980665989139844133934".to_owned();
	let mut big_int = str_to_big(_str);

    let mut len = 233;
    while len > 230 { 
        println!("{:}", big_int.to_string());

        call(&big_int, &mut pool.get_conn().unwrap().unwrap());

		big_int = big_int - 1u64;
		len     = big_int.bits();
	}
}

fn call(big_int: &num_bigint::BigUint,  conn: &mut mysql::Conn) {
	let secp = Secp256k1::new();
	if let Some(pri_comp) =  private_from_big(&big_int, true) {
			 let pub_key = Address::p2pkh(&pri_comp.public_key(&secp), pri_comp.network);
			 if has_balance(pub_key.to_string()) {
			 	store(conn, pri_comp.to_wif(), pub_key.to_string());
			    println!("priv: {:}, pub_key: {:}", pri_comp.to_wif(), pub_key);	 
			 }
			 let pub_key = Address::p2shwpkh(&pri_comp.public_key(&secp), pri_comp.network);
			 if has_balance(pub_key.to_string()) {
			 	store(conn, pri_comp.to_wif(), pub_key.to_string());
			    println!("priv: {:}, pub_key: {:}", pri_comp.to_wif(), pub_key);	 
			 }	
			 
		};

		if let Some(pri_uncomp) =  private_from_big(&big_int, false) {
			let pub_key = Address::p2pkh(&pri_uncomp.public_key(&secp), pri_uncomp.network);
			if has_balance(pub_key.to_string()) {
				store(conn, pri_uncomp.to_wif(), pub_key.to_string());
				println!("priv: {:}, pub_key: {:}", pri_uncomp.to_wif(), pub_key);
			}
			let pub_key = Address::p2shwpkh(&pri_uncomp.public_key(&secp), pri_uncomp.network);
			if has_balance(pub_key.to_string()) {
				store(conn, pri_uncomp.to_wif(), pub_key.to_string());
				println!("priv: {:}, pub_key: {:}", pri_uncomp.to_wif(), pub_key);
			}
			
		};

}