#![allow(unused)]


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

use std::thread;
use std::sync::Mutex;

const NTHREADS: usize = 11;

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
   //read_file_prime();
   //let args: Vec<String> = env::args().collect();
   //let i: usize = args[1].parse().unwrap();

  // read_by_range(i);
   read_by_range_async_all();
}

fn read_file() {
	use std::fs::File;
	use std::io::BufReader; 
	use std::io::BufRead;

	let pool = mysql::Pool::new("mysql://root:password@localhost:3307/wallet").unwrap();

	let file = File::open("C:/Users/HENDRY/Documents/numeros_primos/periodic_table.txt").unwrap();
	let mut reader = BufReader::new(file);

    loop {
        let mut number_str = String::new();
        if let Ok(n) =  reader.read_line(&mut number_str) {
        	if n == 0 { break; }

        	let mut hasher = Sha256::new();
        	//let str_c = number_str.clone();

			// write input message
			hasher.input(&number_str.clone().into_bytes());
			let result  = &hasher.result().to_vec();

			let str_hex = hex::encode(result);
			println!("{:} => {:}", number_str, &str_hex);

		    let mut big_int = str_to_big(str_hex, 16u32);
		    call(&big_int, &mut pool.get_conn().unwrap().unwrap());  		        	           
        }
    }

}

fn read_file_prime() {
	use std::fs::File;
	use std::io::BufReader; 
	use std::io::BufRead;

	let pool = mysql::Pool::new("mysql://test:test@/wallet").unwrap();

	let file = File::open("C:/Users/HENDRY/Documents/numeros_primos/De_900G_a_1T_part172.txt").unwrap();
	let mut reader = BufReader::new(file);

    loop {
        let mut numbers_str = String::new();
        if let Ok(_) =  reader.read_line(&mut numbers_str) {
            let list:Vec<&str> = numbers_str.split("\t").collect();
        	for n_str in list {
		   	   print!("{:} ", n_str.to_string());
		   	   if n_str.to_string().contains("\r\n"){
                  continue;
		   	   }

		       let mut big_int = str_to_big(n_str.to_string(), 10u32);
		       call(&big_int, &mut pool.get_conn().unwrap().unwrap());  		
		    }
			println!("");       	           
        }
    }

}

/*


	 		let list: Vec<(String, String)> = vec![ 
	            ("36326929956805080917198740394882480895143524600985274992771006747580603486145".to_owned(), "40867796201405716031848582944242791007036465176108434366867382591028163402330".to_owned()),
	            ("40867796201405716031848582944242791007036465176108434366867382591028175109889".to_owned(), "49949528690606986261148268042963411230822346326354753115060134277923310825070".to_owned()),
	            ("49949528690606986261148268042963411230822346326354753115060134277923314963305".to_owned(), "54490394935207621375798110592323721342715286901477912489156510121370884536440".to_owned()),
	            ("54490394935207621375798110592323721342715286901477912489156510121370888673252".to_owned(), "63572127424408891605097795691044341566501168051724231237349261808266031959180".to_owned()),
	            ("63572127424408891605097795691044341566501168051724231237349261808266037025565".to_owned(), "68112993669009526719747638240404651678394108626847390611445637651713605670550".to_owned()),
	            ("68112993669009526719747638240404651678394108626847390611445637651713610862802".to_owned(), "72653859913610161834397480789764961790287049201970549985542013495161179381920".to_owned()),
	            ("77194726158210796949047323339125271902179989777093709359638389338608758339510".to_owned(), "81735592402811432063697165888485582014072930352216868733734765182056326804660".to_owned()),
	            ("86276458647412067178347008438888462079378761358813335694269665472374489422226".to_owned(), "95358191136613337407646693536566512349751752077586346856023892712399047938770".to_owned()),
	            ("90817324892012702292996851023034510594105648262487013873821655122077567387785".to_owned(), "99899057381213972522296536085926822461644692652709506230120268555846621650140".to_owned()),
	            ("104439923625814607636946378707428529799286343128795272668145578379551014047250".to_owned(), "108980789870415242751596221184647442685430573802955824978313020242741769072880".to_owned()),
	            ("111251222992715560308921142549645508283961922051594736365803047501698180327189".to_owned(), "121251222992715560308921142459327597741377044090517404665361208164465555928565".to_owned()) // 111251222992715560308921142549645508283961922051594736365803047501698180327189
			];

*/

/*               
	//0  => 70  :  1180591620717411303424 - 1180591620717412009896 => 2361183241434822606848
	1  => 80  :  36326929956805080917198740394882480895143524600985274992771006747580589690960 - 36326929956805080917198740394882480895143524600985274992771006747580599341151
	2  => 90  :  40867796201405716031848582944242791007036465176108434366867382591028163402330 - 40867796201405716031848582944242791007036465176108434366867382591028170971764
	3  => 100 :  45408662446006351146498425493603101118929405751231593740963758434475737113700 - 45408662446006351146498425493603101118929405751231593740963759434475737113700
	4  => 110 :  49949528690606986261148268042963411230822346326354753115060134277923310825070 - 59031261179808256490447953141684031454608227476601071863252885964818458972706
*(	5  => 120 :  54490394935207621375798110592323721342715286901477912489156510121370884536440 - 59031261179808256490447953141684031454608227476601071863252885964818458276052
	6  => 130 :  59031261179808256490447953141684031454608227476601071863252885964818458247810 - 59031261179808256490447953141684031454608227476601071863252885964818459611629
	7  => 140 :  63572127424408891605097795691044341566501168051724231237349261808266031959180 - 63572127424408891605097795691044341566501168051724231237349261808266032821041
	8  => 150 :  68112993669009526719747638240404651678394108626847390611445637651713605670550 - 68112993669009526719747638240404651678394108626847390611445637651713606655045
	9  => 160 :  72653859913610161834397480789764961790287049201970549985542013495161179381920 - 72653859913610161834397480789764961790287049201970549985542013495161179409813
	10 => 170 :  77194726158210796949047323339125271902179989777093709359638389338608753093290 - 77194726158210796949047323339125271902179989777093709359638389338608753941730
	11 => 180 :  81735592402811432063697165888485582014072930352216868733734765182056326804660 - 77194726158210796949047323339125271902179989777093709359638389338608753276005
	12 => 190 :  86276458647412067178347008437845892125965870927340028107831141025503900516030 - 81735592402811432063697165888485582014072930352216868733734765182056326808497
	13 => 200 :  90817324892012702292996850987206202237858811502463187481927516868951474227400 - 86276458647412067178347008437845892125965870927340028107831141025503901149069
	14 => 210 :  95358191136613337407646693536566512349751752077586346856023892712399047938770 - 90817324892012702292996850987206202237858811502463187481927516868951475143881
	15 => 220 :  99899057381213972522296536085926822461644692652709506230120268555846621650140 - 99899057381213972522296536085926822461644692652709506230120268555846621656677
	16 => 230 :  104439923625814607636946378707428529799286343128795272668145578379551014047250 - 104439923625814607636946378635287132573537633227832665604216644399294201989816
	17 => 240 :  108980789870415242751596221184647442685430573802955824978313020242741769072880 - 108980789870415242751596221184647442685430573802955824978313020242741769086056
	18 => 245 :  111251222992715560308921142549645508283961922051594736365803047501698180327189 - 111251222992715560308921142459327597741377044090517404665361208164465562799287
    
*/

fn read_by_range_async_all(){
	 let mut children = vec![];
		 	
	 for i in 0..NTHREADS {

	 	children.push(thread::spawn(move || {

 		let list: Vec<(String, String)> = vec![ 
            ("115792089237316195423570985008687906843269984665640564039457584007913131063685".to_owned(), "115792089237316195423570985008687906853269984665640564039457584007913129639936".to_owned()),
            ("115792089237316195423570985008687906853269984665640564039457584007913131063685".to_owned(), "115792089237316195423570985008687906953269984665640564039457584007913129639936".to_owned()),
            ("115792089237316195423570985008687906953269984665640564039457584007913131063685".to_owned(), "115792089237316195423570985008687907053269984665640564039457584007913129639936".to_owned()),
            ("115792089237316195423570985008687907053269984665640564039457584007913131063685".to_owned(), "115792089237316195423570985008687907153269984665640564039457584007913129639936".to_owned()),
            ("115792089237316195423570985008687907153269984665640564039457584007913131063685".to_owned(), "115792089237316195423570985008687907253269984665640564039457584007913129639936".to_owned()),
            ("115792089237316195423570985008687907253269984665640564039457584007913131063685".to_owned(), "115792089237316195423570985008687907353269984665640564039457584007913129639936".to_owned()),
            ("115792089237316195423570985008687907353269984665640564039457584007913131063685".to_owned(), "115792089237316195423570985008687907453269984665640564039457584007913129639936".to_owned()),
            ("115792089237316195423570985008687907453269984665640564039457584007913131063685".to_owned(), "115792089237316195423570985008687907553269984665640564039457584007913129639936".to_owned()),
            ("115792089237316195423570985008687907553269984665640564039457584007913131063685".to_owned(), "115792089237316195423570985008687907653269984665640564039457584007913129639936".to_owned()),
            ("115792089237316195423570985008687907653269984665640564039457584007913131063685".to_owned(), "115792089237316195423570985008687907753269984665640564039457584007913129639936".to_owned()),
            ("115792089237316195423570985008687907753269984665640564039457584007913131063685".to_owned(), "115792089237316195423570985008687907853269984665640564039457584007913129639936".to_owned()) // 111251222992715560308921142549645508283961922051594736365803047501698180327189
		];
		 	let tup = &list[i];
		    let _str  = &tup.0;
		    let _str_end  = &tup.1;

		    let pool = mysql::Pool::new("mysql://root:password@localhost:3306/wallet").unwrap();

			let mut big_int     = str_to_big(_str.to_string(), 10u32);
			let mut big_int_end = str_to_big(_str_end.to_string(), 10u32);
		     
		    while big_int < big_int_end { 
		        println!("{:}", big_int.to_string());

		        call(&big_int, &mut pool.get_conn().unwrap().unwrap());

				big_int = big_int + 1u64;
			}
		}));
	}
	for child in children {
        // Wait for the thread to finish. Returns a result.
        let _ = child.join();
    }
}/*               
	//0  => 70  :  1180591620717411303424 - 1180591620717412009896 => 2361183241434822606848
	1  => 80  :  36326929956805080917198740394882480895143524600985274992771006747580589690960 - 36326929956805080917198740394882480895143524600985274992771006747580599341151
	2  => 90  :  40867796201405716031848582944242791007036465176108434366867382591028163402330 - 40867796201405716031848582944242791007036465176108434366867382591028170971764
	3  => 100 :  45408662446006351146498425493603101118929405751231593740963758434475737113700 - 45408662446006351146498425493603101118929405751231593740963759434475737113700
	4  => 110 :  49949528690606986261148268042963411230822346326354753115060134277923310825070 - 59031261179808256490447953141684031454608227476601071863252885964818458972706
*(	5  => 120 :  54490394935207621375798110592323721342715286901477912489156510121370884536440 - 59031261179808256490447953141684031454608227476601071863252885964818458276052
	6  => 130 :  59031261179808256490447953141684031454608227476601071863252885964818458247810 - 59031261179808256490447953141684031454608227476601071863252885964818459611629
	7  => 140 :  63572127424408891605097795691044341566501168051724231237349261808266031959180 - 63572127424408891605097795691044341566501168051724231237349261808266032821041
	8  => 150 :  68112993669009526719747638240404651678394108626847390611445637651713605670550 - 68112993669009526719747638240404651678394108626847390611445637651713606655045
	9  => 160 :  72653859913610161834397480789764961790287049201970549985542013495161179381920 - 72653859913610161834397480789764961790287049201970549985542013495161179409813
	10 => 170 :  77194726158210796949047323339125271902179989777093709359638389338608753093290 - 77194726158210796949047323339125271902179989777093709359638389338608753941730
	11 => 180 :  81735592402811432063697165888485582014072930352216868733734765182056326804660 - 77194726158210796949047323339125271902179989777093709359638389338608753276005
	12 => 190 :  86276458647412067178347008437845892125965870927340028107831141025503900516030 - 81735592402811432063697165888485582014072930352216868733734765182056326808497
	13 => 200 :  90817324892012702292996850987206202237858811502463187481927516868951474227400 - 86276458647412067178347008437845892125965870927340028107831141025503901149069
	14 => 210 :  95358191136613337407646693536566512349751752077586346856023892712399047938770 - 90817324892012702292996850987206202237858811502463187481927516868951475143881
	15 => 220 :  99899057381213972522296536085926822461644692652709506230120268555846621650140 - 99899057381213972522296536085926822461644692652709506230120268555846621656677
	16 => 230 :  104439923625814607636946378635287132573537633227832665604216644399294195361510 - 104439923625814607636946378635287132573537633227832665604216644399294201989816
	17 => 240 :  108980789870415242751596221184647442685430573802955824978313020242741769072880 - 108980789870415242751596221184647442685430573802955824978313020242741769086056
	18 => 245 :  111251222992715560308921142459327597741377044090517404665361208164465555928565 - 111251222992715560308921142459327597741377044090517404665361208164465562799287
    
*/

fn read_by_range(i: usize){

	let list: Vec<(String, String)> = vec![ 
            ("36326929956805080917198740394882480895143524600985274992771006747580599341151".to_owned(), "40867796201405716031848582944242791007036465176108434366867382591028163402330".to_owned()),
            ("40867796201405716031848582944242791007036465176108434366867382591028170971764".to_owned(), "49949528690606986261148268042963411230822346326354753115060134277923310825070".to_owned()),
            ("59031261179808256490447953141684031454608227476601071863252885964818458972706".to_owned(), "54490394935207621375798110592323721342715286901477912489156510121370884536440".to_owned()),
            ("59031261179808256490447953141684031454608227476601071863252885964818559611629".to_owned(), "63572127424408891605097795691044341566501168051724231237349261808266031959180".to_owned()),
            ("63572127424408891605097795691044341566501168051724231237349261808266032888076".to_owned(), "68112993669009526719747638240404651678394108626847390611445637651713605670550".to_owned()),
            ("68112993669009526719747638240404651678394108626847390611445637651713606722082".to_owned(), "72653859913610161834397480789764961790287049201970549985542013495161179381920".to_owned()),
            ("77194726158210796949047323339125271902179989777093709359638389338608754200624".to_owned(), "81735592402811432063697165888485582014072930352216868733734765182056326804660".to_owned()),
            ("86276458647412067178347008438888462079378761358813335694269665472374485280785".to_owned(), "95358191136613337407646693536566512349751752077586346856023892712399047938770".to_owned()),
            ("90817324892012702292996851023034510594105648262487013873821655122077563266474".to_owned(), "99899057381213972522296536085926822461644692652709506230120268555846621650140".to_owned()),
            ("104439923625814607636946378707428529799286343128795272668145578379551009906169".to_owned(), "108980789870415242751596221184647442685430573802955824978313020242741769072880".to_owned()),
            ("111251222992715560308921142549645508283961922051594736365803047501698176184480".to_owned(), "121251222992715560308921142459327597741377044090517404665361208164465555928565".to_owned())
	    ];
	
    let pool = mysql::Pool::new("mysql://root:password@localhost:3306/wallet").unwrap();

    let tup      = &list[i];
    let _str     = &tup.0;
    let _str_end = &tup.1;
	let mut big_int     = str_to_big(_str.to_string(), 10u32);
	let mut big_int_end = str_to_big(_str_end.to_string(), 10u32);

     
    while big_int < big_int_end { 
        println!("{:}", big_int.to_string());

        call(&big_int, &mut pool.get_conn().unwrap().unwrap());

		big_int = big_int + 1u64;
	}
}

fn read_by_range_increment(i: usize){

	let list: Vec<(String, String)> = vec![ 
            ("36326929956805080917198740394882480895143524600985274992771006776500496882105".to_owned(), "40867796201405716031848582944242791007036465176108434366867382591028163402330".to_owned()),
            ("40867796201405716031848582944242791007036465176108434366867434325998170889645".to_owned(), "49949528690606986261148268042963411230822346326354753115060134277923310825070".to_owned()),
            ("59031261179808256490447953141684031454608227476601071863252885964818458972706".to_owned(), "54490394935207621375798110592323721342715286901477912489156510121370884536440".to_owned()),
            ("59031261179808256490447953141684031454608227476601071863252885964818559611629".to_owned(), "63572127424408891605097795691044341566501168051724231237349261808266031959180".to_owned()),
            ("63572127424408891605097795691044341566501168051724231237349261808266032821041".to_owned(), "68112993669009526719747638240404651678394108626847390611445637651713605670550".to_owned()),
            ("68112993669009526719747638240404651678394108626847390611445637651713606655045".to_owned(), "72653859913610161834397480789764961790287049201970549985542013495161179381920".to_owned()),
            ("77194726158210796949047323339126015074376222466764557234361243767233215663105".to_owned(), "81735592402811432063697165888485582014072930352216868733734765182056326804660".to_owned()),
            ("86276458647412067178347008440353169323005907704363045454003031422400888886341".to_owned(), "95358191136613337407646693536566512349751752077586346856023892712399047938770".to_owned()),
            ("90817324892012702292996851031875023833670225509301572954394730433739816093448".to_owned(), "99899057381213972522296536085926822461644692652709506230120268555846621650140".to_owned()),
            ("104439923625814607636946378724541137927576107291469431981385055668414805346868".to_owned(), "108980789870415242751596221184647442685430573802955824978313020242741769072880".to_owned()),
            ("120251223002715560308921142549645508283961922051594736365803047501698184818965".to_owned(), "121251222992715560308921142459327597741377044090517404665361208164465555928565".to_owned())
	       // 115792089237316195423570985008687907853269984665640564039457584007913129639936
	           
	    ];
	
    let pool = mysql::Pool::new("mysql://test:test@/wallet").unwrap();

    let tup      = &list[i];
    let _str     = &tup.0;
    let _str_end = &tup.1;
	let mut big_int     = str_to_big(_str.to_string(), 10u32);
	let mut big_int_end = str_to_big(_str_end.to_string(), 10u32);

	let big_int_inc = str_to_big("7741377044090517404665361696220321608976550".to_string(), 10u32);

    while big_int < big_int_end { 
        println!("{:}", big_int.to_string());
        call(&big_int, &mut pool.get_conn().unwrap().unwrap());
		big_int = big_int + &big_int_inc;
	}
}

fn read_every_bit(){
	
    let pool = mysql::Pool::new("mysql://test:test@/wallet").unwrap();

    let _str = "5653910607290829854666552002377339250647948470001980665989139844133934".to_owned();
	let mut big_int = str_to_big(_str, 16u32);

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
		 if exist_address(conn, pub_key.to_string()) {
		 	store(conn, pri_comp.to_wif(), pub_key.to_string());
		    println!("priv: {:}, pub_key: {:}", pri_comp.to_wif(), pub_key);	 
		 }
		 let pub_key = Address::p2shwpkh(&pri_comp.public_key(&secp), pri_comp.network);
		 if exist_address(conn, pub_key.to_string()) {
		 	store(conn, pri_comp.to_wif(), pub_key.to_string());
		    println!("priv: {:}, pub_key: {:}", pri_comp.to_wif(), pub_key);	 
		 }	
		 
	};

	if let Some(pri_uncomp) =  private_from_big(&big_int, false) {
		let pub_key = Address::p2pkh(&pri_uncomp.public_key(&secp), pri_uncomp.network);
		if exist_address(conn, pub_key.to_string()) {
			store(conn, pri_uncomp.to_wif(), pub_key.to_string());
			println!("priv: {:}, pub_key: {:}", pri_uncomp.to_wif(), pub_key);
		}
		let pub_key = Address::p2shwpkh(&pri_uncomp.public_key(&secp), pri_uncomp.network);
		if exist_address(conn, pub_key.to_string()) {
			store(conn, pri_uncomp.to_wif(), pub_key.to_string());
			println!("priv: {:}, pub_key: {:}", pri_uncomp.to_wif(), pub_key);
		}
		
	};

}
