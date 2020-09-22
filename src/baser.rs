/*                                             
Convert a String to a base58 btc address       
*/                                             


//use bitcoin::util::base58;                                                     


pub fn convert2base58(datan: String) -> String {
	 let datan = datan; //"00c26193e66572aaf12f80897073ee7eced11ccd352e6b8a8d".to_string();              
	 let testing = hex::decode(datan).unwrap();                                                 
	 let testing =  bitcoin::util::base58::encode_slice(&testing); 
	testing
}

