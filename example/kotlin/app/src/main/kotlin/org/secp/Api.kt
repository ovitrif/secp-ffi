package org.secp

object Secp {
    fun generateKeyPair(): KeyPair {
        return org.secp.generateKeyPair()
    }
    fun signMessage(message: String, privateKey: List<UByte>): List<UByte> {
        return org.secp.signMessage(message, privateKey)
    }
    fun verifyMessage(signatureCompact: List<UByte>, message: String, privateKey: List<UByte>): Boolean {
        return org.secp.verifyMessage(signatureCompact, message, privateKey)
    }
}
