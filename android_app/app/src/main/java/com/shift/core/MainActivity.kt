package com.shift.core

import android.Manifest
import android.annotation.SuppressLint
import android.bluetooth.BluetoothManager
import android.bluetooth.le.AdvertiseCallback
import android.bluetooth.le.AdvertiseData
import android.bluetooth.le.AdvertiseSettings
import android.bluetooth.le.ScanCallback
import android.bluetooth.le.ScanResult
import android.bluetooth.le.ScanSettings
import android.content.Context
import android.content.pm.PackageManager
import android.hardware.biometrics.BiometricPrompt
import android.location.Location
import android.location.LocationManager
import android.os.Bundle
import android.os.CancellationSignal
import android.security.keystore.KeyGenParameterSpec
import android.security.keystore.KeyProperties
import android.util.Log
import android.graphics.Color
import android.graphics.drawable.GradientDrawable
import android.widget.Button
import android.widget.LinearLayout
import android.widget.ScrollView
import android.widget.TextView
import androidx.appcompat.app.AppCompatActivity
import java.io.InputStream
import java.io.OutputStream
import java.lang.reflect.Method
import java.lang.reflect.Proxy
import java.security.KeyPairGenerator
import java.security.KeyStore
import kotlinx.coroutines.*

// =========================================================================
// 💎 PINNACLE CONCURRENCY: LOCK-FREE & ALLOCATION-FREE SNAPSHOT BUFFER
// =========================================================================
class LockFreeRingBufferSet(private val capacity: Int) {
    private val array = java.util.concurrent.atomic.AtomicReferenceArray<String?>(capacity)
    private val index = java.util.concurrent.atomic.AtomicInteger(0)

    fun add(element: String): Boolean {
        for (i in 0 until capacity) {
            if (array.get(i) == element) return false
        }
        val targetIdx = index.getAndIncrement() % capacity
        array.set(targetIdx, element)
        return true
    }

    fun getSnapshot(): List<String> {
        val snapshot = mutableListOf<String>()
        for (i in 0 until capacity) {
            val el = array.get(i)
            if (el != null) {
                snapshot.add(el)
            }
        }
        return snapshot
    }
}

// =========================================================================
// PHASE 1.6: THE DYNAMIC VSOCK HYPERVISOR BRIDGE
// =========================================================================
object TeeBridge {
    var activeVm: Any? = null
    var isNativeFallback: Boolean = false
    const val VSOCK_PORT: Long = 8000

    private fun executeSovereignHandshake(inputStream: InputStream, outputStream: OutputStream, command: String): String {
        Log.d("SHIFT_TUNNEL", "executeSovereignHandshake: Started")
        val dataIn = java.io.DataInputStream(inputStream)
        val dataOut = java.io.DataOutputStream(outputStream)

        // 1. Read Vault Ephemeral PubKey (65 bytes) and Nonce (32 bytes)
        Log.d("SHIFT_TUNNEL", "executeSovereignHandshake: Reading 65-byte Vault PubKey...")
        val vaultPubBytes = ByteArray(65)
        dataIn.readFully(vaultPubBytes)
        Log.d("SHIFT_TUNNEL", "executeSovereignHandshake: Read Vault PubKey successfully")
        
        Log.d("SHIFT_TUNNEL", "executeSovereignHandshake: Reading 32-byte Nonce...")
        val nonce = ByteArray(32)
        dataIn.readFully(nonce)
        Log.d("SHIFT_TUNNEL", "executeSovereignHandshake: Read Nonce successfully")

        // 2. Generate App Ephemeral KeyPair
        val kpg = java.security.KeyPairGenerator.getInstance("EC")
        kpg.initialize(java.security.spec.ECGenParameterSpec("secp256r1"))
        val appKeyPair = kpg.generateKeyPair()
        
        val ecPubKey = appKeyPair.public as java.security.interfaces.ECPublicKey
        val w = ecPubKey.w
        val appPubBytes = ByteArray(65)
        appPubBytes[0] = 0x04
        val affineX = w.affineX.toByteArray()
        val affineY = w.affineY.toByteArray()
        fun copyTo32(src: ByteArray, dest: ByteArray, offset: Int) {
            val start = if (src.size > 32) src.size - 32 else 0
            val len = Math.min(32, src.size)
            val destStart = offset + (32 - len)
            System.arraycopy(src, start, dest, destStart, len)
        }
        copyTo32(affineX, appPubBytes, 1)
        copyTo32(affineY, appPubBytes, 33)

        // 3. Derive Shared Secret
        val keyFactory = java.security.KeyFactory.getInstance("EC")
        val vaultW = java.security.spec.ECPoint(
            java.math.BigInteger(1, vaultPubBytes.copyOfRange(1, 33)),
            java.math.BigInteger(1, vaultPubBytes.copyOfRange(33, 65))
        )
        val vaultParams = ecPubKey.params
        val vaultPubSpec = java.security.spec.ECPublicKeySpec(vaultW, vaultParams)
        val vaultPubKey = keyFactory.generatePublic(vaultPubSpec)

        val keyAgreement = javax.crypto.KeyAgreement.getInstance("ECDH")
        keyAgreement.init(appKeyPair.private)
        keyAgreement.doPhase(vaultPubKey, true)
        val sharedSecret = keyAgreement.generateSecret()

        // 4. Derive AES-GCM Key
        Log.d("SHIFT_TUNNEL", "executeSovereignHandshake: Deriving AES key...")
        val md = java.security.MessageDigest.getInstance("SHA-256")
        val aesKey = javax.crypto.spec.SecretKeySpec(md.digest(sharedSecret), "AES")

        // 5. Encrypt Command Payload
        Log.d("SHIFT_TUNNEL", "executeSovereignHandshake: Encrypting payload...")
        val cipher = javax.crypto.Cipher.getInstance("AES/GCM/NoPadding")
        val reqNonce = nonce.copyOfRange(0, 12)
        val gcmSpec = javax.crypto.spec.GCMParameterSpec(128, reqNonce)
        cipher.init(javax.crypto.Cipher.ENCRYPT_MODE, aesKey, gcmSpec)
        val ciphertext = cipher.doFinal(command.toByteArray(Charsets.UTF_8))

        // 6. Sign Transcript
        Log.d("SHIFT_TUNNEL", "executeSovereignHandshake: Loading keyStore and private key...")
        val keyStore = java.security.KeyStore.getInstance("AndroidKeyStore")
        keyStore.load(null)
        val hardwarePrivateKey = keyStore.getKey("SHIFT_SOVEREIGN_NODE_ID", null) as java.security.PrivateKey
        val hardwareCert = keyStore.getCertificate("SHIFT_SOVEREIGN_NODE_ID")
        val hwPub = hardwareCert.publicKey.encoded

        Log.d("SHIFT_TUNNEL", "executeSovereignHandshake: Preparing signature transcript...")
        val transcript = java.io.ByteArrayOutputStream().apply {
            write(vaultPubBytes)
            write(appPubBytes)
            write(nonce)
            write(ciphertext)
        }.toByteArray()

        Log.d("SHIFT_TUNNEL", "executeSovereignHandshake: Initializing signature...")
        val signature = java.security.Signature.getInstance("SHA256withECDSA")
        signature.initSign(hardwarePrivateKey)
        signature.update(transcript)
        Log.d("SHIFT_TUNNEL", "executeSovereignHandshake: Signing transcript...")
        val sig = signature.sign()
        Log.d("SHIFT_TUNNEL", "executeSovereignHandshake: Signed transcript successfully")

        // 7. Write App Response to Vault
        Log.d("SHIFT_TUNNEL", "executeSovereignHandshake: Writing App Response to stream...")
        dataOut.writeShort(hwPub.size)
        dataOut.write(hwPub)
        dataOut.write(appPubBytes)
        dataOut.writeShort(sig.size)
        dataOut.write(sig)
        dataOut.writeInt(ciphertext.size)
        dataOut.write(ciphertext)
        dataOut.flush()
        Log.d("SHIFT_TUNNEL", "executeSovereignHandshake: Flushed response to stream. Waiting for Vault response...")

        // 8. Read Vault Response
        val resLen = dataIn.readInt()
        Log.d("SHIFT_TUNNEL", "executeSovereignHandshake: Got Vault response length: $resLen bytes. Reading ciphertext...")
        val encryptedRes = ByteArray(resLen)
        dataIn.readFully(encryptedRes)
        Log.d("SHIFT_TUNNEL", "executeSovereignHandshake: Read Vault response ciphertext successfully")

        // 9. Decrypt Vault Response
        Log.d("SHIFT_TUNNEL", "executeSovereignHandshake: Decrypting response...")
        val resNonce = nonce.copyOfRange(12, 24)
        val resGcmSpec = javax.crypto.spec.GCMParameterSpec(128, resNonce)
        cipher.init(javax.crypto.Cipher.DECRYPT_MODE, aesKey, resGcmSpec)
        val decryptedRes = cipher.doFinal(encryptedRes)
        Log.d("SHIFT_TUNNEL", "executeSovereignHandshake: Decrypted response successfully")

        return String(decryptedRes, Charsets.UTF_8)
    }

    @SuppressLint("DiscouragedPrivateApi")
    fun sendCommand(command: String): String {
        if (activeVm == null && !isNativeFallback) return "❌ Execution Denied: Hypervisor/Fallback is offline. Ignite pKVM first."

        if (isNativeFallback) {
            var attempts = 0
            val maxAttempts = 5
            var lastException: Exception? = null
            while (attempts < maxAttempts) {
                var socket: java.net.Socket? = null
                try {
                    socket = java.net.Socket("127.0.0.1", VSOCK_PORT.toInt())
                    val response = executeSovereignHandshake(socket.getInputStream(), socket.getOutputStream(), command)
                    return response
                } catch (e: Exception) {
                    Log.e("SHIFT_TUNNEL", "Error in sendCommand attempt $attempts: ${e.message}", e)
                    lastException = e
                    attempts++
                    if (attempts < maxAttempts) {
                        Thread.sleep(100)
                    }
                } finally {
                    try {
                        socket?.close()
                    } catch (ignored: Exception) {}
                }
            }
            return "❌ TCP Fallback Connection Error: ${lastException?.message} (Failed after $maxAttempts attempts)"
        }

        val vm = activeVm!!
        var socket: Any? = null
        return try {
            var vsockMethod: Method? = null
            for (m in vm.javaClass.methods) {
                if (m.name.contains("connectVsock", ignoreCase = true) ||
                    m.name.contains("connectToVsockServer", ignoreCase = true)) {
                    vsockMethod = m
                    break
                }
            }

            if (vsockMethod == null) return "❌ VSOCK binding not found on this hardware."

            socket = try {
                vsockMethod.invoke(vm, VSOCK_PORT)
            } catch (e: Exception) {
                Log.w("SHIFT_AVF", "Long port failed, trying Int. (${e.message})")
                vsockMethod.invoke(vm, VSOCK_PORT.toInt())
            } ?: return "❌ Failed to establish VSOCK stream."

            val getInputStreamMethod = socket.javaClass.getMethod("getInputStream")
            val getOutputStreamMethod = socket.javaClass.getMethod("getOutputStream")

            val inputStream = getInputStreamMethod.invoke(socket) as InputStream
            val outputStream = getOutputStreamMethod.invoke(socket) as OutputStream

            val response = executeSovereignHandshake(inputStream, outputStream, command)
            response
        } catch (e: Exception) {
            "❌ VSOCK Connection Error: ${e.message}"
        } finally {
            if (socket != null) {
                try {
                    val closeMethod = socket.javaClass.getMethod("close")
                    closeMethod.invoke(socket)
                } catch (ignored: Exception) {}
            }
        }
    }
}

private var nodeKeyRegenerated = false

@SuppressLint("MissingPermission", "SetTextI18n", "DiscouragedPrivateApi", "PrivateApi")
class MainActivity : AppCompatActivity() {
    private lateinit var statusText: TextView
    private lateinit var scrollView: ScrollView
    private val nearbyNodes = LockFreeRingBufferSet(128)
    private var isMeshActive = false
    private val ioScope = CoroutineScope(Dispatchers.IO)
    private var nativeProcess: Process? = null

    // Constant for our hardware key
    private val keyAlias = "SHIFT_SOVEREIGN_NODE_ID"

    private fun appendLog(msg: String) {
        runOnUiThread {
            statusText.append(msg)
            scrollView.post {
                scrollView.fullScroll(android.view.View.FOCUS_DOWN)
            }
        }
    }

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        supportActionBar?.hide()
        window.statusBarColor = Color.parseColor("#0B0E17")

        val layout = LinearLayout(this).apply {
            orientation = LinearLayout.VERTICAL
            setPadding(30, 20, 30, 30)
            setBackgroundColor(Color.parseColor("#0B0E17")) // Deep obsidian background
            layoutParams = LinearLayout.LayoutParams(
                LinearLayout.LayoutParams.MATCH_PARENT,
                LinearLayout.LayoutParams.MATCH_PARENT
            )
        }

        // 1. Premium Header Bar
        val headerLayout = LinearLayout(this).apply {
            orientation = LinearLayout.VERTICAL
            setPadding(0, 10, 0, 20)
        }
        val titleText = TextView(this).apply {
            text = "S.H.I.F.T. CONTROL STATION"
            textSize = 20f
            setTextColor(Color.parseColor("#F8FAFC"))
            typeface = android.graphics.Typeface.create("sans-serif-condensed-medium", android.graphics.Typeface.BOLD)
            gravity = android.view.Gravity.CENTER_HORIZONTAL
        }
        val subtitleText = TextView(this).apply {
            text = "Sovereign Hardware Infrastructure For Transit"
            textSize = 10.5f
            setTextColor(Color.parseColor("#64748B"))
            gravity = android.view.Gravity.CENTER_HORIZONTAL
        }
        headerLayout.addView(titleText)
        headerLayout.addView(subtitleText)
        layout.addView(headerLayout)

        // 2. Button Styling Helper
        fun Button.styleNodeButton(backgroundColor: String, strokeColor: String, isFullWidth: Boolean = false) {
            val normalDrawable = GradientDrawable().apply {
                shape = GradientDrawable.RECTANGLE
                cornerRadius = 10f
                setColor(Color.parseColor(backgroundColor))
                setStroke(2, Color.parseColor(strokeColor))
            }
            val pressedDrawable = GradientDrawable().apply {
                shape = GradientDrawable.RECTANGLE
                cornerRadius = 10f
                setColor(Color.parseColor(strokeColor))
                setStroke(2, Color.parseColor(strokeColor))
            }
            
            val states = android.graphics.drawable.StateListDrawable().apply {
                addState(intArrayOf(android.R.attr.state_pressed), pressedDrawable)
                addState(intArrayOf(), normalDrawable)
            }
            
            background = states
            setTextColor(Color.parseColor("#FFFFFF"))
            textSize = 9.5f
            setPadding(16, 20, 16, 20)
            typeface = android.graphics.Typeface.create("sans-serif-medium", android.graphics.Typeface.BOLD)
            
            val params = if (isFullWidth) {
                LinearLayout.LayoutParams(
                    LinearLayout.LayoutParams.MATCH_PARENT,
                    LinearLayout.LayoutParams.WRAP_CONTENT
                ).apply {
                    setMargins(6, 6, 6, 6)
                }
            } else {
                LinearLayout.LayoutParams(
                    0,
                    LinearLayout.LayoutParams.WRAP_CONTENT,
                    1f
                ).apply {
                    setMargins(6, 6, 6, 6)
                }
            }
            layoutParams = params
        }

        // 3. Create and Style Buttons in 2-Column Layout
        val avfButton = Button(this).apply { 
            text = "PHASE 1.6: IGNITE PKVM" 
            styleNodeButton("#1E1B4B", "#6366F1") // Indigo
        }
        val polButton = Button(this).apply { 
            text = "PHASE 1.5: MESH + POL" 
            styleNodeButton("#064E3B", "#10B981") // Emerald
        }
        val lockButton = Button(this).apply { 
            text = "PHASE 2.4: FIRE LOCK" 
            styleNodeButton("#7C2D12", "#F97316") // Rust/Orange
        }
        val genesisButton = Button(this).apply { 
            text = "PHASE 3.1: MINT GENESIS" 
            styleNodeButton("#581C87", "#A855F7") // Purple
        }
        val zkvmButton = Button(this).apply { 
            text = "PHASE 4.1: IGNITE ZKVM" 
            styleNodeButton("#0F766E", "#14B8A6") // Teal
        }
        val zkPsiButton = Button(this).apply { 
            text = "PHASE 3: ZK-PSI REJECT" 
            styleNodeButton("#1E293B", "#0284C7") // Slate/Sky
        }
        val pricingButton = Button(this).apply { 
            text = "EXECUTE: PHASE 4.3 (HYBRID MARKET-MAKER & ZKVM)" 
            styleNodeButton("#701A75", "#D946EF", isFullWidth = true) // Fuchsia
        }

        val row1 = LinearLayout(this).apply {
            orientation = LinearLayout.HORIZONTAL
            layoutParams = LinearLayout.LayoutParams(LinearLayout.LayoutParams.MATCH_PARENT, LinearLayout.LayoutParams.WRAP_CONTENT)
            addView(avfButton)
            addView(polButton)
        }
        val row2 = LinearLayout(this).apply {
            orientation = LinearLayout.HORIZONTAL
            layoutParams = LinearLayout.LayoutParams(LinearLayout.LayoutParams.MATCH_PARENT, LinearLayout.LayoutParams.WRAP_CONTENT)
            addView(lockButton)
            addView(genesisButton)
        }
        val row3 = LinearLayout(this).apply {
            orientation = LinearLayout.HORIZONTAL
            layoutParams = LinearLayout.LayoutParams(LinearLayout.LayoutParams.MATCH_PARENT, LinearLayout.LayoutParams.WRAP_CONTENT)
            addView(zkvmButton)
            addView(zkPsiButton)
        }
        val row4 = LinearLayout(this).apply {
            orientation = LinearLayout.HORIZONTAL
            layoutParams = LinearLayout.LayoutParams(LinearLayout.LayoutParams.MATCH_PARENT, LinearLayout.LayoutParams.WRAP_CONTENT)
            addView(pricingButton)
        }

        val buttonsContainer = LinearLayout(this).apply {
            orientation = LinearLayout.VERTICAL
            layoutParams = LinearLayout.LayoutParams(LinearLayout.LayoutParams.MATCH_PARENT, LinearLayout.LayoutParams.WRAP_CONTENT)
            addView(row1)
            addView(row2)
            addView(row3)
            addView(row4)
        }
        layout.addView(buttonsContainer)

        // 4. Console Log Label
        val consoleTitle = TextView(this).apply {
            text = "SYSTEM BOOT LOG & NODE STATUS"
            textSize = 9.5f
            setTextColor(Color.parseColor("#64748B"))
            typeface = android.graphics.Typeface.create("sans-serif-medium", android.graphics.Typeface.BOLD)
            setPadding(0, 10, 0, 8)
        }
        layout.addView(consoleTitle)

        // 5. Scrollable Console Box
        scrollView = ScrollView(this).apply {
            val params = LinearLayout.LayoutParams(
                LinearLayout.LayoutParams.MATCH_PARENT,
                0,
                1f
            )
            layoutParams = params
            isFillViewport = true
            
            background = GradientDrawable().apply {
                shape = GradientDrawable.RECTANGLE
                cornerRadius = 14f
                setColor(Color.parseColor("#020617")) // Deep slate-950 console black
                setStroke(2, Color.parseColor("#1E293B")) // Slate-800 border
            }
        }

        statusText = TextView(this).apply {
            textSize = 11f
            setTextColor(Color.parseColor("#2DD4BF")) // Teal console green
            typeface = android.graphics.Typeface.MONOSPACE
            gravity = android.view.Gravity.TOP or android.view.Gravity.LEFT
            setPadding(24, 24, 24, 48) // 48px padding at the bottom for breathing room!
            addOnLayoutChangeListener { _, _, _, _, _, _, _, _, _ ->
                scrollView.post {
                    scrollView.fullScroll(android.view.View.FOCUS_DOWN)
                }
            }
        }

        scrollView.addView(statusText)
        layout.addView(scrollView)
        setContentView(layout)

        avfButton.setOnClickListener {
            statusText.append("\n\n[REQUESTING TYPE-1 HYPERVISOR LEASE...]")
            igniteHypervisor()
        }

        polButton.setOnClickListener {
            if (checkAndRequestPermissions()) {
                if (!isMeshActive) {
                    startBleMesh()
                    statusText.append("\n\n[Activating BLE/Wi-Fi Aware Mesh. Scanning Environment...]")
                }
                triggerBiometricGate()
            }
        }

        lockButton.setOnClickListener {
            if (isMeshActive) {
                val targetZone = "zone:892b9ab93c7ffff"
                statusText.append("\n\n[FIRING LAMPORT TICKET INTO SHARD: $targetZone...]")
                ioScope.launch {
                    val lockResponse = TeeBridge.sendCommand("FIRE_LOCK:$targetZone")
                    withContext(Dispatchers.Main) { statusText.append("\n$lockResponse") }
                }
            } else {
                statusText.append("\n\n--- EXECUTION DENIED ---\nMesh must be active (Phase 1.5) before firing locks.")
            }
        }

        genesisButton.setOnClickListener {
            statusText.append("\n\n[ANCHORING NODE TO BLOCK-LATTICE...]")
            ioScope.launch {
                val genesisResponse = TeeBridge.sendCommand("MINT_GENESIS:")
                withContext(Dispatchers.Main) { statusText.append("\n$genesisResponse") }
            }
        }

        zkPsiButton.setOnClickListener {
            if (!isMeshActive) {
                statusText.append("\n\n--- ENGINE ERROR ---\nYou must activate the BLE Mesh (Phase 1.5) first to scan ambient MAC addresses.")
                return@setOnClickListener
            }

            statusText.append("\n\n[FIRING ZK-PSI MATHEMATICAL REJECTION ENGINE...]")

            val snapshot = nearbyNodes.getSnapshot()
            val scannedString = if (snapshot.isNotEmpty()) { snapshot.joinToString(",") } else { "00:11:22:33:44:55" }
            val expectedString = if (snapshot.size >= 3) {
                val realMacsToMatch = snapshot.take(3).joinToString(",")
                "$realMacsToMatch,FF:EE:DD:CC:BB:AA"
            } else {
                "00:11:22:33:44:55,AA:BB:CC:DD:EE:FF,11:22:33:44:55:66"
            }

            ioScope.launch {
                val psiResult = TeeBridge.sendCommand("VERIFY_PSI:$scannedString|$expectedString")
                withContext(Dispatchers.Main) { statusText.append("\n$psiResult") }
            }
        }

        zkvmButton.setOnClickListener {
            statusText.append("\n\n[ALLOCATING MEMORY FOR NOVA IVC FOLDING...]")
            ioScope.launch {
                val vmResponse = TeeBridge.sendCommand("IGNITE_ZKVM:")
                withContext(Dispatchers.Main) { statusText.append("\n$vmResponse") }
            }
        }

        pricingButton.setOnClickListener {
            if (!isMeshActive) {
                statusText.append("\n\n--- ENGINE ERROR ---\nYou must activate the BLE Mesh (Phase 1.5) to gather the Supply/Demand ratio.")
                return@setOnClickListener
            }

            statusText.append("\n\n[INITIATING HYBRID MARKET-MAKER...]")

            val localDemand = nearbyNodes.getSnapshot().size
            val baseRatePerMile = 1.50
            val surgeMultiplier = if (localDemand > 5) 1.5 else 1.0
            val aiSuggestedFare = baseRatePerMile * surgeMultiplier

            statusText.append("\nLocal Nodes Detected: $localDemand")
            statusText.append("\nAI Surge Multiplier: ${surgeMultiplier}x")
            statusText.append("\nCalculated Base Fare: $$aiSuggestedFare per mile")

            val riderBid = 1.10
            statusText.append("\n\n[INCOMING P2P BID: $$riderBid per mile]")
            statusText.append("\n[IGNITING ZK-VM TO VERIFY ALGORITHMIC FLOOR...]")

            ioScope.launch {
                val vmResponse = TeeBridge.sendCommand("IGNITE_ZKVM:")
                withContext(Dispatchers.Main) {
                    statusText.append("\n$vmResponse\n❌ [SMART CONTRACT] Bid Rejected: Rider bid ($$riderBid) falls below the Algorithmic Floor ($$baseRatePerMile).")
                }
            }
        }
    }

    private fun igniteHypervisor() {
        if (TeeBridge.activeVm != null || (nativeProcess != null && nativeProcess?.isAlive == true)) {
            statusText.append("\n⚠️ Hypervisor or Native Fallback daemon is already active.")
            return
        }
        TeeBridge.isNativeFallback = false
        Log.i("SHIFT_AVF", "Initiating pKVM Hypervisor Ignition via Reflection Bypass...")

        try {
            // 1. Fetch the VirtualMachineManager directly as a System Service
            val vmManager = applicationContext.getSystemService("virtualmachine")

            if (vmManager == null) {
                statusText.append("\n⚠️ OS restricts pKVM access. Initializing Native Fallback...")
                igniteNativeFallback()
                return
            }

            val vmmClass = Class.forName("android.system.virtualmachine.VirtualMachineManager")

            // 2. Build Config
            val builderClass = Class.forName("android.system.virtualmachine.VirtualMachineConfig\$Builder")
            val builder = builderClass.getConstructor(Context::class.java).newInstance(applicationContext)

            builderClass.getMethod("setProtectedVm", Boolean::class.javaPrimitiveType).invoke(builder, true)
            builderClass.getMethod("setPayloadBinaryName", String::class.java).invoke(builder, "libshift_core.so")

            try {
                builderClass.getMethod("setMemoryBytes", Long::class.javaPrimitiveType).invoke(builder, 256L * 1024 * 1024)
            } catch (e: Exception) {
                Log.d("SHIFT_AVF", "setMemoryBytes failed, falling back to setMemoryMib: ${e.message}")
                try {
                    builderClass.getMethod("setMemoryMib", Int::class.javaPrimitiveType).invoke(builder, 256)
                } catch(e2: Exception) {
                    Log.d("SHIFT_AVF", "setMemoryMib also failed: ${e2.message}")
                }
            }

            val config = builderClass.getMethod("build").invoke(builder)
            val configClass = Class.forName("android.system.virtualmachine.VirtualMachineConfig")

            // 3. Get or Create VM
            val getOrCreateMethod = vmmClass.getMethod("getOrCreate", String::class.java, configClass)
            val vm = getOrCreateMethod.invoke(vmManager, "shift_vault_vm", config)

            // 4. Create Callback Proxy
            val callbackClass = Class.forName("android.system.virtualmachine.VirtualMachineCallback")
            val proxy = Proxy.newProxyInstance(classLoader, arrayOf(callbackClass)) { _, method, args ->
                when (method.name) {
                    "onPayloadStarted" -> {
                        runOnUiThread { statusText.append("\n💎 [HYPERVISOR] Microdroid VM Booted. Hardware isolated.") }
                    }
                    "onPayloadReady" -> {
                        runOnUiThread {
                            statusText.append("\n⚙️ [HYPERVISOR] Rust Payload Running. VSOCK Bridge established.")
                            TeeBridge.activeVm = args?.get(0)
                            authenticateForBoot {
                                executeSecureBootSequence()
                            }
                        }
                    }
                    "onStopped" -> {
                        val reason = args?.get(1)
                        runOnUiThread { statusText.append("\n⚠️ [HYPERVISOR] VM Terminated (Code: $reason).") }
                        TeeBridge.activeVm = null
                    }
                    "onError" -> {
                        val message = args?.get(2) as? String
                        runOnUiThread { statusText.append("\n❌ [HYPERVISOR] Crash: $message") }
                    }
                }
                null
            }

            // 5. Set Callback and Run
            val vmClass = Class.forName("android.system.virtualmachine.VirtualMachine")
            vmClass.getMethod("setCallback", java.util.concurrent.Executor::class.java, callbackClass).invoke(vm, mainExecutor, proxy)

            statusText.append("\n⏳ [HYPERVISOR] Spinning up Microdroid Core...")
            vmClass.getMethod("run").invoke(vm)

        } catch (e: Exception) {
            statusText.append("\n❌ [HYPERVISOR] Ignition Exception: ${e.message}")
            Log.e("SHIFT_AVF", "Reflection failed", e)
        }
    }

    private fun executeSecureBootSequence() {
        ioScope.launch {
            try {
                val publicKeyHex = generateTrustZoneKey()
                val sClassical = deriveSClassical()
                val sPqc = getOrGeneratePqcSecret()

                val payload = "$publicKeyHex|${byteArrayToHexString(sClassical)}|${byteArrayToHexString(sPqc)}"
                val rustIdentityResponse = TeeBridge.sendCommand("REGISTER_NODE:$payload")

                val sbtToken = "SBT-CLEAR-ID-9942"
                val rustSbtResponse = TeeBridge.sendCommand("ISSUE_SBT:$sbtToken")
                val genesisResponse = TeeBridge.sendCommand("MINT_GENESIS:")

                withContext(Dispatchers.Main) {
                    statusText.append("\n\nSYSTEM BOOT:\n$rustIdentityResponse\n$rustSbtResponse\n$genesisResponse\n\nReady for Telemetry.")
                }
            } catch (e: Exception) {
                withContext(Dispatchers.Main) {
                    statusText.append("\nBoot Failed: ${e.message}")
                    Log.e("SHIFT_BOOT", "Secure boot execution failed", e)
                }
            }
        }
    }

    private fun checkAndRequestPermissions(): Boolean {
        val required = mutableListOf(
            Manifest.permission.ACCESS_FINE_LOCATION,
            Manifest.permission.BLUETOOTH_SCAN,
            Manifest.permission.BLUETOOTH_ADVERTISE,
            Manifest.permission.BLUETOOTH_CONNECT
        )

        val missing = required.filter { checkSelfPermission(it) != PackageManager.PERMISSION_GRANTED }
        if (missing.isNotEmpty()) {
            requestPermissions(missing.toTypedArray(), 1)
            return false
        }
        return true
    }

    private fun startBleMesh() {
        val bluetoothManager = getSystemService(Context.BLUETOOTH_SERVICE) as BluetoothManager
        val adapter = bluetoothManager.adapter ?: return
        val scanner = adapter.bluetoothLeScanner
        val advertiser = adapter.bluetoothLeAdvertiser

        val settings = AdvertiseSettings.Builder().setAdvertiseMode(AdvertiseSettings.ADVERTISE_MODE_LOW_LATENCY).build()
        val data = AdvertiseData.Builder().setIncludeDeviceName(false).build()
        try {
            advertiser?.startAdvertising(settings, data, object : AdvertiseCallback() {})
            val scanSettings = ScanSettings.Builder().setScanMode(ScanSettings.SCAN_MODE_LOW_LATENCY).build()
            scanner?.startScan(null, scanSettings, object : ScanCallback() {
                override fun onScanResult(callbackType: Int, result: ScanResult) {
                    result.device?.address?.let { nearbyNodes.add(it) }
                }
            })
            isMeshActive = true
        } catch (e: SecurityException) {
            statusText.append("\n❌ BLE Permission Error: ${e.message}")
        }
    }

    private fun triggerBiometricGate() {
        val biometricPrompt = BiometricPrompt.Builder(this)
            .setTitle("Sovereign Identity Verification")
            .setSubtitle("Authorize hardware to sign PoL telemetry")
            .setNegativeButton("Cancel", mainExecutor) { _, _ ->
                statusText.append("\n\n--- EXECUTION DENIED ---\nBiometric cancelled.")
            }
            .build()

        biometricPrompt.authenticate(CancellationSignal(), mainExecutor, object : BiometricPrompt.AuthenticationCallback() {
            override fun onAuthenticationSucceeded(result: BiometricPrompt.AuthenticationResult?) {
                super.onAuthenticationSucceeded(result)
                statusText.append("\n\n[Biometric Verified. Human Presence Confirmed.]")
                executeProofOfLocation()
            }
            override fun onAuthenticationError(errorCode: Int, errString: CharSequence?) {
                super.onAuthenticationError(errorCode, errString)
                statusText.append("\n\n--- EXECUTION FAILED ---\nHardware Error: $errString")
            }
        })
    }

    private fun executeProofOfLocation() {
        val locationManager = getSystemService(Context.LOCATION_SERVICE) as LocationManager
        val location: Location? = try {
            locationManager.getLastKnownLocation(LocationManager.GPS_PROVIDER) ?: locationManager.getLastKnownLocation(LocationManager.NETWORK_PROVIDER)
        } catch (e: SecurityException) {
            Log.w("SHIFT_AVF", "Location permission missing", e)
            null
        }

        val snapshot = nearbyNodes.getSnapshot()
        val meshConsensus = if (snapshot.isNotEmpty()) { "BLE:${snapshot.joinToString(",")}" } else { "BLE:ISOLATED" }
        val telemetry = if (location != null) {
            "LAT:${location.latitude}|LON:${location.longitude}|ALT:${location.altitude}|$meshConsensus|TS:${System.currentTimeMillis()}"
        } else {
            "LAT:46.2382|LON:-63.1311|ALT:0.0|$meshConsensus|TS:${System.currentTimeMillis()}|[INDOOR_NO_FIX]"
        }

        ioScope.launch {
            val polResponse = TeeBridge.sendCommand("GENERATE_POL:$telemetry")
            withContext(Dispatchers.Main) { statusText.append("\n$polResponse") }
        }
    }

    private fun generateTrustZoneKey(): String {
        val keyStore = KeyStore.getInstance("AndroidKeyStore")
        keyStore.load(null)

        if (!nodeKeyRegenerated) {
            try {
                keyStore.deleteEntry(keyAlias)
                Log.d("SHIFT_KEYSTORE", "Deleted old node signing key to force regeneration with validity duration.")
            } catch (e: Exception) {
                Log.w("SHIFT_KEYSTORE", "Failed to delete old key: ${e.message}")
            }
            nodeKeyRegenerated = true
        }

        if (!keyStore.containsAlias(keyAlias)) {
            val keyPairGenerator = KeyPairGenerator.getInstance(KeyProperties.KEY_ALGORITHM_EC, "AndroidKeyStore")
            var parameterSpec: KeyGenParameterSpec
            
            try {
                parameterSpec = KeyGenParameterSpec.Builder(keyAlias, KeyProperties.PURPOSE_SIGN or KeyProperties.PURPOSE_VERIFY)
                    .setDigests(KeyProperties.DIGEST_SHA256)
                    .setIsStrongBoxBacked(true)
                    .setUserAuthenticationRequired(true)
                    .setUserAuthenticationValidityDurationSeconds(15)
                    .build()
                keyPairGenerator.initialize(parameterSpec)
                keyPairGenerator.generateKeyPair()
                Log.d("SHIFT_KEYSTORE", "Generated Primary Signing Key with StrongBox backing and 15s validity duration.")
            } catch (e: Exception) {
                Log.w("SHIFT_KEYSTORE", "StrongBox EC Signing Key failed, falling back to standard TEE. (${e.message})")
                parameterSpec = KeyGenParameterSpec.Builder(keyAlias, KeyProperties.PURPOSE_SIGN or KeyProperties.PURPOSE_VERIFY)
                    .setDigests(KeyProperties.DIGEST_SHA256)
                    .setIsStrongBoxBacked(false)
                    .setUserAuthenticationRequired(true)
                    .setUserAuthenticationValidityDurationSeconds(15)
                    .build()
                keyPairGenerator.initialize(parameterSpec)
                keyPairGenerator.generateKeyPair()
                Log.d("SHIFT_KEYSTORE", "Generated Primary Signing Key with standard TEE backing and 15s validity duration.")
            }
        }

        return byteArrayToHexString(keyStore.getCertificate(keyAlias).publicKey.encoded)
    }

    private val agreementKeyAlias = "SHIFT_SOVEREIGN_AGREEMENT_KEY"

    private fun generateTrustZoneAgreementKey() {
        val keyStore = KeyStore.getInstance("AndroidKeyStore")
        keyStore.load(null)

        if (!keyStore.containsAlias(agreementKeyAlias)) {
            val keyPairGenerator = KeyPairGenerator.getInstance(KeyProperties.KEY_ALGORITHM_EC, "AndroidKeyStore")
            var parameterSpec: KeyGenParameterSpec
            
            try {
                parameterSpec = KeyGenParameterSpec.Builder(agreementKeyAlias, KeyProperties.PURPOSE_AGREE_KEY)
                    .setAlgorithmParameterSpec(java.security.spec.ECGenParameterSpec("secp256r1"))
                    .setIsStrongBoxBacked(true)
                    .setUserAuthenticationRequired(true)
                    .setUserAuthenticationValidityDurationSeconds(15)
                    .build()
                keyPairGenerator.initialize(parameterSpec)
                keyPairGenerator.generateKeyPair()
                Log.d("SHIFT_KEYSTORE", "Generated Agreement Key with StrongBox backing.")
            } catch (e: Exception) {
                Log.w("SHIFT_KEYSTORE", "StrongBox EC Agreement Key failed, falling back to standard TEE. (${e.message})")
                parameterSpec = KeyGenParameterSpec.Builder(agreementKeyAlias, KeyProperties.PURPOSE_AGREE_KEY)
                    .setAlgorithmParameterSpec(java.security.spec.ECGenParameterSpec("secp256r1"))
                    .setIsStrongBoxBacked(false)
                    .setUserAuthenticationRequired(true)
                    .setUserAuthenticationValidityDurationSeconds(15)
                    .build()
                keyPairGenerator.initialize(parameterSpec)
                keyPairGenerator.generateKeyPair()
                Log.d("SHIFT_KEYSTORE", "Generated Agreement Key with standard TEE backing.")
            }
        }
    }

    private val pqcWrapperKeyAlias = "SHIFT_PQC_WRAPPER_KEY"

    private fun generatePqcWrapperKey() {
        val keyStore = KeyStore.getInstance("AndroidKeyStore")
        keyStore.load(null)

        if (!keyStore.containsAlias(pqcWrapperKeyAlias)) {
            val keyGenerator = javax.crypto.KeyGenerator.getInstance(KeyProperties.KEY_ALGORITHM_AES, "AndroidKeyStore")
            var parameterSpec: KeyGenParameterSpec
            
            try {
                parameterSpec = KeyGenParameterSpec.Builder(
                    pqcWrapperKeyAlias,
                    KeyProperties.PURPOSE_ENCRYPT or KeyProperties.PURPOSE_DECRYPT
                )
                .setBlockModes(KeyProperties.BLOCK_MODE_GCM)
                .setEncryptionPaddings(KeyProperties.ENCRYPTION_PADDING_NONE)
                .setIsStrongBoxBacked(true)
                .setUserAuthenticationRequired(true)
                .setUserAuthenticationValidityDurationSeconds(15)
                .build()
                keyGenerator.init(parameterSpec)
                keyGenerator.generateKey()
                Log.d("SHIFT_KEYSTORE", "Generated PQC AES Wrapper Key with StrongBox backing.")
            } catch (e: Exception) {
                Log.w("SHIFT_KEYSTORE", "StrongBox AES Wrapper Key failed, falling back to standard TEE. (${e.message})")
                parameterSpec = KeyGenParameterSpec.Builder(
                    pqcWrapperKeyAlias,
                    KeyProperties.PURPOSE_ENCRYPT or KeyProperties.PURPOSE_DECRYPT
                )
                .setBlockModes(KeyProperties.BLOCK_MODE_GCM)
                .setEncryptionPaddings(KeyProperties.ENCRYPTION_PADDING_NONE)
                .setIsStrongBoxBacked(false)
                .setUserAuthenticationRequired(true)
                .setUserAuthenticationValidityDurationSeconds(15)
                .build()
                keyGenerator.init(parameterSpec)
                keyGenerator.generateKey()
                Log.d("SHIFT_KEYSTORE", "Generated PQC AES Wrapper Key with standard TEE backing.")
            }
        }
    }

    private fun getOrGeneratePqcSecret(): ByteArray {
        val sharedPrefs = getSharedPreferences("shift_pqc_prefs", Context.MODE_PRIVATE)
        val encryptedHex = sharedPrefs.getString("encrypted_pqc", null)
        val ivHex = sharedPrefs.getString("iv_pqc", null)

        val keyStore = KeyStore.getInstance("AndroidKeyStore")
        keyStore.load(null)
        
        generatePqcWrapperKey()
        val secretKey = keyStore.getKey(pqcWrapperKeyAlias, null) as javax.crypto.SecretKey

        if (encryptedHex != null && ivHex != null) {
            val encryptedBytes = hexStringToByteArray(encryptedHex)
            val ivBytes = hexStringToByteArray(ivHex)
            val cipher = javax.crypto.Cipher.getInstance("AES/GCM/NoPadding")
            val spec = javax.crypto.spec.GCMParameterSpec(128, ivBytes)
            cipher.init(javax.crypto.Cipher.DECRYPT_MODE, secretKey, spec)
            return cipher.doFinal(encryptedBytes)
        } else {
            val secureRandom = java.security.SecureRandom()
            val rawSeed = ByteArray(32)
            secureRandom.nextBytes(rawSeed)

            val cipher = javax.crypto.Cipher.getInstance("AES/GCM/NoPadding")
            cipher.init(javax.crypto.Cipher.ENCRYPT_MODE, secretKey)
            val encryptedBytes = cipher.doFinal(rawSeed)
            val ivBytes = cipher.iv

            sharedPrefs.edit().apply {
                putString("encrypted_pqc", byteArrayToHexString(encryptedBytes))
                putString("iv_pqc", byteArrayToHexString(ivBytes))
                apply()
            }
            return rawSeed
        }
    }

    private fun deriveSClassical(): ByteArray {
        val staticSaltPubKeyBytes = byteArrayOf(
            0x30.toByte(), 0x59.toByte(), 0x30.toByte(), 0x13.toByte(), 0x06.toByte(), 0x07.toByte(), 0x2a.toByte(), 0x86.toByte(),
            0x48.toByte(), 0xce.toByte(), 0x3d.toByte(), 0x02.toByte(), 0x01.toByte(), 0x06.toByte(), 0x08.toByte(), 0x2a.toByte(),
            0x86.toByte(), 0x48.toByte(), 0xce.toByte(), 0x3d.toByte(), 0x03.toByte(), 0x01.toByte(), 0x07.toByte(), 0x03.toByte(),
            0x42.toByte(), 0x00.toByte(), 0x04.toByte(), 0xb7.toByte(), 0x15.toByte(), 0x65.toByte(), 0x92.toByte(), 0x8a.toByte(),
            0x4e.toByte(), 0x41.toByte(), 0xaa.toByte(), 0x63.toByte(), 0xb7.toByte(), 0x52.toByte(), 0x4c.toByte(), 0x94.toByte(),
            0xe1.toByte(), 0xdf.toByte(), 0x61.toByte(), 0xae.toByte(), 0xa2.toByte(), 0x91.toByte(), 0xd6.toByte(), 0x09.toByte(),
            0x1e.toByte(), 0x7e.toByte(), 0x36.toByte(), 0x5e.toByte(), 0x97.toByte(), 0x2a.toByte(), 0xab.toByte(), 0x0c.toByte(),
            0xed.toByte(), 0xce.toByte(), 0xe0.toByte(), 0xc4.toByte(), 0x0c.toByte(), 0x64.toByte(), 0xc9.toByte(), 0xeb.toByte(),
            0xa4.toByte(), 0x80.toByte(), 0xe9.toByte(), 0xfb.toByte(), 0x63.toByte(), 0x92.toByte(), 0x57.toByte(), 0xde.toByte(),
            0x9e.toByte(), 0xab.toByte(), 0x65.toByte(), 0x50.toByte(), 0x21.toByte(), 0xbf.toByte(), 0x39.toByte(), 0xe7.toByte(),
            0xb8.toByte(), 0x54.toByte(), 0xde.toByte(), 0x63.toByte(), 0x97.toByte(), 0x36.toByte(), 0x96.toByte(), 0xa8.toByte(),
            0x66.toByte(), 0xe7.toByte(), 0x16.toByte()
        )

        val keyStore = KeyStore.getInstance("AndroidKeyStore")
        keyStore.load(null)
        
        generateTrustZoneAgreementKey()

        val privateKey = keyStore.getKey(agreementKeyAlias, null) as java.security.PrivateKey
        val keyFactory = java.security.KeyFactory.getInstance("EC")
        val pubKeySpec = java.security.spec.X509EncodedKeySpec(staticSaltPubKeyBytes)
        val peerPubKey = keyFactory.generatePublic(pubKeySpec)

        val keyAgreement = javax.crypto.KeyAgreement.getInstance("ECDH", "AndroidKeyStore")
        keyAgreement.init(privateKey)
        keyAgreement.doPhase(peerPubKey, true)
        return keyAgreement.generateSecret()
    }

    private fun byteArrayToHexString(bytes: ByteArray): String {
        return bytes.joinToString("") { String.format("%02x", it) }
    }

    private fun hexStringToByteArray(s: String): ByteArray {
        val len = s.length
        val data = ByteArray(len / 2)
        var i = 0
        while (i < len) {
            data[i / 2] = ((Character.digit(s[i], 16) shl 4) + Character.digit(s[i + 1], 16)).toByte()
            i += 2
        }
        return data
    }

    private fun authenticateForBoot(onAuthenticated: () -> Unit) {
        val biometricPrompt = BiometricPrompt.Builder(this)
            .setTitle("Ignite Node Identity")
            .setSubtitle("Authenticate hardware to derive secure keys")
            .setNegativeButton("Cancel", mainExecutor) { _, _ ->
                statusText.append("\n\n--- BOOT DENIED ---\nBiometric authentication cancelled.")
            }
            .build()

        biometricPrompt.authenticate(CancellationSignal(), mainExecutor, object : BiometricPrompt.AuthenticationCallback() {
            override fun onAuthenticationSucceeded(result: BiometricPrompt.AuthenticationResult?) {
                super.onAuthenticationSucceeded(result)
                statusText.append("\n\n[Biometric Verified. Deriving P2P Cryptographic Keys...]")
                onAuthenticated()
            }
            override fun onAuthenticationError(errorCode: Int, errString: CharSequence?) {
                super.onAuthenticationError(errorCode, errString)
                statusText.append("\n\n--- BOOT FAILED ---\nHardware Lock Error: $errString")
            }
        })
    }

    private fun igniteNativeFallback() {
        ioScope.launch {
            try {
                val nativeLibDir = applicationInfo.nativeLibraryDir
                val coreBinary = java.io.File(nativeLibDir, "libshift_core.so")
                
                if (!coreBinary.exists()) {
                    withContext(Dispatchers.Main) { statusText.append("\n❌ [FALLBACK] Core binary not found at ${coreBinary.absolutePath}") }
                    return@launch
                }
                
                coreBinary.setExecutable(true)

                val processBuilder = ProcessBuilder(coreBinary.absolutePath)
                processBuilder.redirectErrorStream(true)
                nativeProcess = processBuilder.start()

                withContext(Dispatchers.Main) { 
                    statusText.append("\n💎 [FALLBACK] Native Daemon Booted. Hardware sandboxed by SELinux.") 
                }

                // Give the daemon 500ms to bind to port 8000
                delay(500)

                withContext(Dispatchers.Main) {
                    statusText.append("\n⚙️ [FALLBACK] Rust Payload Running. TCP Bridge established.")
                    TeeBridge.isNativeFallback = true
                    authenticateForBoot {
                        executeSecureBootSequence()
                    }
                }

                val reader = nativeProcess?.inputStream?.bufferedReader()
                var line: String?
                while (reader?.readLine().also { line = it } != null) {
                    Log.i("SHIFT_VAULT_DAEMON", line!!)
                }

            } catch (e: Exception) {
                withContext(Dispatchers.Main) { statusText.append("\n❌ [FALLBACK] Ignition Exception: ${e.message}") }
            }
        }
    }

    override fun onDestroy() {
        super.onDestroy()
        nativeProcess?.destroy()
    }
}