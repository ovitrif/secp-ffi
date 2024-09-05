package org.secp

fun main() {
    println()
    println("secp256k1 from rust")
    try {
        val keyPair = Secp.generateKeyPair()
        println("KeyPair")
        println("├── Private Key: ${keyPair.privateKey}")
        println("└── Public Key: ${keyPair.publicKey}")

        val message = "Hello World!"
        val signature = Secp.signMessage(message, keyPair.privateKey)
        println("Signature: $signature")

        val isVerified = Secp.verifyMessage(signature, message, keyPair.publicKey)
        println("isVerified: $isVerified")

    } catch (e: Exception) {
        e.printStackTrace()
    }
}
