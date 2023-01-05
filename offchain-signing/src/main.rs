use cryptoxide::ed25519;
use renvm_sig::keccak256;
use renvm_sig::hash_message;

fn main() {

    //Edit the data according to in erc20 or pair contract
    let domain_separator = "c9d478deb03a5aa3b82f643641de6b02a2d7ba6115bd706807c8f10f0799ff40"; // change it according to your project
    let permit_type_hash = "6e71edae12b1b97f4d1f60370fef10105fa2faae0126114a169c64845d6126c9";  // change it according to your project
    let owner = "Key::Account(1c9756cd83c7f03a024e0a434ad64e138fab7c9c49d6241e8b468a44dec3adc5)";  // change it according to your project
    let spender = "Key::Account(1c9756cd83c7f03a024e0a434ad64e138fab7c9c49d6241e8b468a44dec3adc5)"; // change it according to your project
    let value=5;  // change it according to your project
    let nonce=0;  // change it according to your project
    let deadline:u64=99999999999; // change it according to your project

    println!("");

    let data:String = format!("{}{}{}{}{}{}", permit_type_hash, owner, spender, value, nonce, deadline);
    println!("The Cancatenation of the Data: {:?}",data);
    println!("");

    let hash = keccak256(data.as_bytes());
    println!("The Hash of the Data: {:?}",hash);
    println!("");
    
    let hash_string = hex::encode(hash);
    println!("The hash string form is : {}",hash_string);
    println!("");

    let encode_packed:String = format!("{}{}", domain_separator, hash_string);
    println!("The Cancatenation of the Data2: {:?}",encode_packed);
    println!("");

    let digest=hash_message(encode_packed);
    println!("The digest: {:?}",digest);
    println!("");

    let digest_string = hex::encode(digest);
    println!("The digest_string form is : {}",digest_string);
    println!("");

    let secret= "MC4CAQAwBQYDK2VwBCIEIPPGVic1+UO0UJJJRTHaBkpH/05oaDQacEinXQnKoaIu".as_bytes();
    println!("Secret Key : {:?}",secret);
    println!("");

    let public=ed25519::to_public(secret);
    println!("Public Key : {:?}",public);
    println!("");

    let signature = ed25519::signature_extended(&digest, &secret);
    println!("Signature : {:?}",signature);
    println!("");

    let result:bool=ed25519::verify(&digest,&public,&signature);
    println!("result is : {}",result);
    println!("");

}
