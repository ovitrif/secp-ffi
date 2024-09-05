package org.secp

import kotlin.test.Test
import kotlin.test.assertNotNull

class AppTest {
    @Test fun `test generateKeyPair`() {
        val keyPair = Secp.generateKeyPair()
        assertNotNull(keyPair.privateKey, "privateKey should be generated")
        assertNotNull(keyPair.publicKey, "pubKey should be generated")
    }
}
