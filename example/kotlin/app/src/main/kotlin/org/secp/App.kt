package org.secp

fun main() {
    println()
    println("secp256k1 from rust")
    try {
        val keyPair = Secp.generateKeyPair()
        println("KeyPair")
        println("├── Private Key(${keyPair.privateKey.size}): ${keyPair.privateKey}")
        println("└── Public Key(${keyPair.publicKey.size}): ${keyPair.publicKey}")

        val message = "Hello World!"
        val signature = Secp.signMessage(message, keyPair.privateKey)
        println("Signature(${signature.size}): $signature")

        val isVerified = Secp.verifyMessage(signature, message, keyPair.publicKey)
        println("isVerified: $isVerified")

        val keyPairPeer = Secp.generateKeyPair()
        val ourSharedSecret = Secp.generateSharedSecret(keyPair.privateKey, keyPairPeer.publicKey)
        val theirSharedSecret = Secp.generateSharedSecret(keyPairPeer.privateKey, keyPair.publicKey)
        assert(ourSharedSecret == theirSharedSecret) { "Shared secret is not equal" }
        println("SharedSecret (${ourSharedSecret.length}): $ourSharedSecret")

    } catch (e: Exception) {
        e.printStackTrace()
    }
}
