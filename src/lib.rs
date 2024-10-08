use secp256k1::ecdh::SharedSecret;
use secp256k1::hashes::{sha256, sha256d, Hash};
use secp256k1::rand::rngs::OsRng;
use secp256k1::{ecdsa, Message, PublicKey, Secp256k1, SecretKey};

// region types
pub struct KeyPair {
    private_key: Vec<u8>,
    public_key: Vec<u8>,
}

impl KeyPair {
    fn new(private_key: Vec<u8>, public_key: Vec<u8>) -> Self {
        KeyPair { private_key, public_key }
    }
}
// endregion

// region functions
pub fn generate_key_pair() -> KeyPair {
    let secp = Secp256k1::new();
    let (secret_key, public_key) = secp.generate_keypair(&mut OsRng);
    KeyPair::new(secret_key.secret_bytes().to_vec(), public_key.serialize().to_vec())
}

pub fn sign_message(message_string: String, private_key_bytes: Vec<u8>) -> Vec<u8> {
    let secp = Secp256k1::new();
    let secret_key = SecretKey::from_slice(&private_key_bytes).expect("32 bytes private key");

    let digest = sha256d(message_string);
    let message = Message::from_digest(digest.to_byte_array());

    let signature = secp.sign_ecdsa(&message, &secret_key);
    signature.serialize_compact().to_vec()
}

pub fn verify_message(signature_compact_bytes: Vec<u8>, message_string: String, public_key_bytes: Vec<u8>) -> bool {
    let secp = Secp256k1::verification_only();
    let signature = ecdsa::Signature::from_compact(&signature_compact_bytes).expect("64 bytes compact signature");
    let public_key = PublicKey::from_slice(&public_key_bytes).expect("33 bytes public key");

    let digest = sha256d(message_string);
    let message = Message::from_digest(digest.to_byte_array());

    secp.verify_ecdsa(&message, &signature, &public_key).is_ok()
}

pub fn generate_shared_secret(our_private_key_bytes: Vec<u8>, their_public_key_bytes: Vec<u8>) -> String {
    let secret_key = SecretKey::from_slice(&our_private_key_bytes).expect("32 bytes private key");
    let public_key = PublicKey::from_slice(&their_public_key_bytes).expect("33 bytes public key");

    let secret = SharedSecret::new(&public_key, &secret_key);
    secret.display_secret().to_string()
}

fn sha256d(message_string: String) -> sha256d::Hash {
    sha256::Hash::hash(message_string.as_bytes()).hash_again()
}
// endregion

// region tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_key_pair() {
        let key_pair = generate_key_pair();
        assert_eq!(key_pair.public_key.len(), 33); // Ensure public key length is 33 bytes
        assert_eq!(key_pair.private_key.len(), 32); // Ensure private key length is 32 bytes
        println!("Public Key: {:?}", key_pair.public_key);
        println!("Private Key: {:?}", key_pair.private_key);
    }

    #[test]
    fn test_sign_message() {
        let key_pair = generate_key_pair();
        let message = "Hello World!";
        let signature = sign_message(message.to_string(), key_pair.private_key);
        assert_eq!(signature.len(), 64); // Ensure signature length is 64 bytes
        println!("Message: {}", message);
        println!("Signature: {:?}", signature);
    }

    #[test]
    fn test_verify_message() {
        let key_pair = generate_key_pair();
        let message = "Hello World!";
        let signature = sign_message(message.to_string(), key_pair.private_key);
        let is_verified = verify_message(signature, message.to_string(), key_pair.public_key);
        assert!(is_verified); // Ensure the message is verified
        println!("Message: {}", message);
        println!("IsVerified: {}", is_verified);
    }

    #[test]
    fn test_generate_shared_secret() {
        let our_key_pair = generate_key_pair();
        let their_key_pair = generate_key_pair();
        let our_shared_secret = generate_shared_secret(our_key_pair.private_key.clone(), their_key_pair.public_key.clone());
        let their_shared_secret = generate_shared_secret(their_key_pair.private_key.clone(), our_key_pair.public_key.clone());
        assert_eq!(our_shared_secret, their_shared_secret);
        println!("Our Shared Secret: {}", our_shared_secret);
        println!("Their Shared Secret: {}", their_shared_secret);
    }
}
// endregion

uniffi::include_scaffolding!("secp");
