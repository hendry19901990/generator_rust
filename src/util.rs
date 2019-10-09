use crate::reqwest;

use std::io::Read;

use num_bigint::BigUint;
use bitcoin::util::key::PrivateKey;
use bitcoin::network::constants::Network;

#[derive(Debug, Serialize, Deserialize)]
pub struct AddressResponse {
  pub final_balance: i64,
  pub total_received: i64,
}

pub fn private_from_big(num: &BigUint, compressed: bool) -> Option<PrivateKey>{
	match secp256k1::SecretKey::from_slice(& fill_vec(&num.to_bytes_be()) )  {
		Ok(secret_key) => {
	        return Some(PrivateKey {
	            compressed: compressed,
	            network: Network::Bitcoin,
	            key: secret_key,
	        });
		},
		Err(err) => {
            println!("{:?}", err);
            return None;
		}
	}
}

pub fn str_to_big(str : String, radix: u32) -> BigUint{
   BigUint::parse_bytes(&(*str.into_bytes()), radix).unwrap()
}

pub fn fill_vec(data : &[u8]) -> [u8; 32] {
	let mut result = [0u8; 32];
	let mut i = 0;
	for n in data {
		if i == 32 {
			break
		}
		result[i] = *n;
		i = i + 1;
	}
	return result;
}

pub fn has_balance(addr: String) -> bool {
     let mut response = reqwest::get( &format!("https://blockchain.info/rawaddr/{}", &addr) ).unwrap();
     let mut body = String::new();
     response.read_to_string(&mut body).unwrap();

     if response.status() != 200 {
        return false
     }

     let result: AddressResponse = serde_json::from_str(&body).unwrap();
     //println!("{:?}", result);
     result.final_balance > 0 || result.total_received > 0
}

