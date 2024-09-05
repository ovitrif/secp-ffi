use secp256k1::rand::rngs::OsRng;
use secp256k1::{Secp256k1, Message, SecretKey, PublicKey, ecdsa};
use secp256k1::ffi::CPtr;
use secp256k1::hashes::{sha256, Hash};

pub fn generate_key_pair() -> (Vec<u8>, Vec<u8>) {
    let secp = Secp256k1::new();
    let (secret_key, public_key) = secp.generate_keypair(&mut OsRng);
    (public_key.serialize().to_vec(), secret_key.secret_bytes().to_vec())
}

pub fn sign_message(message_string: String, private_key_bytes: Vec<u8>) -> Vec<u8> {
    let secp = Secp256k1::new();
    let secret_key = SecretKey::from_slice(&private_key_bytes).expect("32 bytes private key");

    let digest = sha256::Hash::hash(message_string.as_bytes());
    let message = Message::from_digest(digest.to_byte_array());

    let signature = secp.sign_ecdsa(&message, &secret_key);
    signature.serialize_compact().to_vec()
}

pub fn verify_message(signature_compact: Vec<u8>, message_string: String, public_key: Vec<u8>) -> bool {
    let secp = Secp256k1::verification_only();
    let signature = ecdsa::Signature::from_compact(&signature_compact).expect("compact signature");
    let public_key = PublicKey::from_slice(&public_key).expect("Valid public key");

    let digest = sha256::Hash::hash(message_string.as_bytes());
    let message = Message::from_digest(digest.to_byte_array());

    secp.verify_ecdsa(&message, &signature, &public_key).is_ok()
}

// region tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_key_pair() {
        let (public_key, private_key) = generate_key_pair();
        assert_eq!(public_key.len(), 33); // Ensure public key length is 33 bytes
        assert_eq!(private_key.len(), 32); // Ensure private key length is 32 bytes
        println!("Public Key: {:?}", public_key);
        println!("Private Key: {:?}", private_key);
    }

    #[test]
    fn test_sign_message() {
        let (public_key, private_key) = generate_key_pair();
        let message = "Hello World!";
        let signature = sign_message(message.to_string(), private_key);
        assert_eq!(signature.len(), 64); // Ensure signature length is 64 bytes
        println!("Message: {}", message);
        println!("Signature: {:?}", signature);
    }

    #[test]
    fn test_verify_message() {
        let (public_key, private_key) = generate_key_pair();
        let message = "Hello World!";
        let signature = sign_message(message.to_string(), private_key);
        let is_verified = verify_message(signature, message.to_string(), public_key);
        assert!(is_verified); // Ensure the message is verified
        println!("Message: {}", message);
        println!("IsVerified: {}", is_verified);
    }
}
// endregion

// region playground
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//         println!("result {}", result);
//     }
// }
// endregion

uniffi::include_scaffolding!("secp");
