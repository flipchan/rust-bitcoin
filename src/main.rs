//Bitcoin in Pure Rust
use rand::thread_rng;
use sha2::{Sha256, Digest};
use secp256k1::{SecretKey, PublicKey};
use std::fmt::Write;
use ripemd160::Ripemd160;

mod baser;


fn base58me(fluff: String) -> String {
	baser::convert2base58(fluff)
}

//https://en.bitcoin.it/wiki/Technical_background_of_version_1_Bitcoin_addresses

/*
0 - Having a private ECDSA key

   18e14a7b6a307f426a94f8114701e7c8e774e7f9a47e2c2035db29a206321725
1 - Take the corresponding public key generated with it (33 bytes, 1 byte 0x02 (y-coord is even), and 32 bytes corresponding to X coordinate)

   0250863ad64a87ae8a2fe83c1af1a8403cb53f53e486d8511dad8a04887e5b2352
2 - Perform SHA-256 hashing on the public key

   0b7c28c9b7290c98d7438e70b3d3f7c848fbd7d1dc194ff83f4f7cc9b1378e98
3 - Perform RIPEMD-160 hashing on the result of SHA-256

   f54a5851e9372b87810a8e60cdd2e7cfd80b6e31
4 - Add version byte in front of RIPEMD-160 hash (0x00 for Main Network)

   00f54a5851e9372b87810a8e60cdd2e7cfd80b6e31
(note that below steps are the Base58Check encoding, which has multiple library options available implementing it)
5 - Perform SHA-256 hash on the extended RIPEMD-160 result

   ad3c854da227c7e99c4abfad4ea41d71311160df2e415e713318c70d67c6b41c
6 - Perform SHA-256 hash on the result of the previous SHA-256 hash

   c7f18fe8fcbed6396741e58ad259b5cb16b7fd7f041904147ba1dcffabf747fd
7 - Take the first 4 bytes of the second SHA-256 hash. This is the address checksum

   c7f18fe8
8 - Add the 4 checksum bytes from stage 7 at the end of extended RIPEMD-160 hash from stage 4. This is the 25-byte binary Bitcoin Address.

   00f54a5851e9372b87810a8e60cdd2e7cfd80b6e31c7f18fe8
9 - Convert the result from a byte string into a base58 string using Base58Check encoding. This is the most commonly used Bitcoin Address format

   1PMycacnJaSqwwJqjawXBErnLsZ7RkXUAs

*/

//get it in printable format
fn pk_to_hex(fluff: [u8; 32]) -> String {

let mut buf = String::new();
 for byte in &fluff {
    // print!("{:x}", byte);
     write!(buf, "{:02x}", byte).unwrap();
 }

	buf
}

//step 7 | get 4 first bytes of a hash
fn first4bytes(inmed: String) -> String {

	let fluff = &inmed.as_bytes()[..8];
	let sparkle_heart = std::str::from_utf8(&fluff).unwrap();
	sparkle_heart.to_string()

}


//generate ecdsa private key
fn generate_pk_key() -> [u8; 32] {

	let pk = SecretKey::random(&mut thread_rng());
	let nyapk = pk.serialize();
	nyapk
}


//append 00 to string
fn ripemd160_to_mainnet(oghash: String) -> String {

  let mut s = String::new();
   s.push_str("00");
	s.push_str(oghash.as_str());


	s
}

//Convert the pk to a pbk
fn get_pbk_from_pk(pkkk: [u8; 32]) {
	let pk = SecretKey::random(&mut thread_rng());
	let pbk = PublicKey::from_secret_key(&pk).serialize_compressed();
let mut buf = String::new();
 for byte in pbk.iter() {
    // print!("{:x}", byte);
     write!(buf, "{:02x}", byte).unwrap();
 }
	println!("buf is: {}", buf);
	let nextone = hashme(&buf);
	let nexx = ripemd160hash(&nextone);
	let pm = ripemd160_to_mainnet(nexx);//step 4
	let step5 = hashme(&pm);
	let step6 = hashme(&step5);
	let step7 = first4bytes(step6);
	println!("first 4 bytes is: {} ", step7);

// 8 - Add the 4 checksum bytes from stage 7 at the end of extended RIPEMD-160 hash from stage 4. This is the 25-byte binary Bitcoin Address.
// So pm + step7

	let mut step8 = String::new();
	step8.push_str(&pm);
	step8.push_str(&step7);
	println!("step8: {}", step8);
//	println!("pm is: {}", pm);
// step 9 base58 encode
//	let step8 = "00f54a5851e9372b87810a8e60cdd2e7cfd80b6e31c7f18fe8".to_string();
	let step9 = base58me(step8);
	println!("step 9: {}", step9)

}


fn hashme(tohash: &String) -> String{
	let mut hasher = Sha256::new();
	hasher.update(tohash);
	let result = hasher.finalize();

let mut buf = String::new();
 for byte in result {
    // print!("{:x}", byte);
     write!(buf, "{:02x}", byte).unwrap();
 }
//	println!("hash is: {}", buf);

	buf

}


fn ripemd160hash(tohash: &String) -> String {
	let mut hasher = Ripemd160::new();
	hasher.update(tohash);
	let result = hasher.finalize();

let mut buf = String::new();
 for byte in result {
     write!(buf, "{:02x}", byte).unwrap();
 }
	buf
//	println!("ripemd160 hash is: {}", buf);
}




fn main() {
	let fluff = pk_to_hex(generate_pk_key());
	println!("Private key is: {}", fluff);
	get_pbk_from_pk(generate_pk_key());
}
