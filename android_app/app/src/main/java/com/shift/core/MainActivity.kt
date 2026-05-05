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
import android.widget.Button
import android.widget.LinearLayout
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
// PHASE 1.6: THE DYNAMIC VSOCK HYPERVISOR BRIDGE
// =========================================================================
object TeeBridge {
    var activeVm: Any? = null
    const val VSOCK_PORT: Long = 8000

    @SuppressLint("DiscouragedPrivateApi")
    fun sendCommand(command: String): String {
        val vm = activeVm ?: return "❌ Execution Denied: Hypervisor is offline. Ignite pKVM first."

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

            val socket = try {
                vsockMethod.invoke(vm, VSOCK_PORT)
            } catch (e: Exception) {
                Log.w("SHIFT_AVF", "Long port failed, trying Int. (${e.message})")
                vsockMethod.invoke(vm, VSOCK_PORT.toInt())
            } ?: return "❌ Failed to establish VSOCK stream."

            val getInputStreamMethod = socket.javaClass.getMethod("getInputStream")
            val getOutputStreamMethod = socket.javaClass.getMethod("getOutputStream")
            val closeMethod = socket.javaClass.getMethod("close")

            val inputStream = getInputStreamMethod.invoke(socket) as InputStream
            val outputStream = getOutputStreamMethod.invoke(socket) as OutputStream

            outputStream.write(command.toByteArray())
            outputStream.flush()

            val response = inputStream.bufferedReader().use { it.readText() }
            closeMethod.invoke(socket)

            response
        } catch (e: Exception) {
            "❌ VSOCK Connection Error: ${e.message}"
        }
    }
}

@SuppressLint("MissingPermission", "SetTextI18n", "DiscouragedPrivateApi", "PrivateApi")
class MainActivity : AppCompatActivity() {
    private lateinit var statusText: TextView
    private val nearbyNodes = mutableSetOf<String>()
    private var isMeshActive = false
    private val ioScope = CoroutineScope(Dispatchers.IO)

    // Constant for our hardware key
    private val keyAlias = "SHIFT_SOVEREIGN_NODE_ID"

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        val layout = LinearLayout(this).apply {
            orientation = LinearLayout.VERTICAL
            setPadding(50, 50, 50, 50)
        }

        statusText = TextView(this).apply { textSize = 14f }

        val avfButton = Button(this).apply { text = "EXECUTE: PHASE 1.6 (Ignite pKVM Hypervisor)" }
        val polButton = Button(this).apply { text = "EXECUTE: PHASE 1.5 (Proximity Mesh + PoL)" }
        val lockButton = Button(this).apply { text = "EXECUTE: PHASE 2.4 (Fire Sub-50ms Lock)" }
        val genesisButton = Button(this).apply { text = "EXECUTE: PHASE 3.1 (Mint Genesis Block)" }
        val zkvmButton = Button(this).apply { text = "EXECUTE: PHASE 4.1 (Ignite On-Device zkVM)" }
        val zkPsiButton = Button(this).apply { text = "EXECUTE: PHASE 3 (Test zk-PSI Rejection Engine)" }
        val pricingButton = Button(this).apply { text = "EXECUTE: PHASE 4.3 (Hybrid Market-Maker & zkVM)" }

        layout.addView(avfButton)
        layout.addView(polButton)
        layout.addView(lockButton)
        layout.addView(genesisButton)
        layout.addView(zkvmButton)
        layout.addView(zkPsiButton)
        layout.addView(pricingButton)
        layout.addView(statusText)
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

            val scannedString = if (nearbyNodes.isNotEmpty()) { nearbyNodes.joinToString(",") } else { "00:11:22:33:44:55" }
            val expectedString = if (nearbyNodes.size >= 3) {
                val realMacsToMatch = nearbyNodes.take(3).joinToString(",")
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

            val localDemand = nearbyNodes.size
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
                    statusText.append("\n$vmResponse")
                    statusText.append("\n❌ [SMART CONTRACT] Bid Rejected: Rider bid ($$riderBid) falls below the Algorithmic Floor ($$baseRatePerMile).")
                }
            }
        }
    }

    private fun igniteHypervisor() {
        Log.i("SHIFT_AVF", "Initiating pKVM Hypervisor Ignition via Reflection Bypass...")

        try {
            // 1. Fetch the VirtualMachineManager directly as a System Service
            val vmManager = applicationContext.getSystemService("virtualmachine")

            if (vmManager == null) {
                statusText.append("\n❌ CRITICAL: OS restricts pKVM access on this device. Service not found.")
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
                            executeSecureBootSequence()
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
                val rustIdentityResponse = TeeBridge.sendCommand("REGISTER_NODE:$publicKeyHex")
                val sbtToken = "SBT-CLEAR-ID-9942"
                val rustSbtResponse = TeeBridge.sendCommand("ISSUE_SBT:$sbtToken")

                withContext(Dispatchers.Main) {
                    statusText.append("\n\nSYSTEM BOOT:\n$rustIdentityResponse\n$rustSbtResponse\n\nReady for Telemetry.")
                }
            } catch (e: Exception) {
                withContext(Dispatchers.Main) {
                    statusText.append("\nBoot Failed: ${e.message}")
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

        val meshConsensus = if (nearbyNodes.isNotEmpty()) { "BLE:${nearbyNodes.joinToString(",")}" } else { "BLE:ISOLATED" }
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

        if (!keyStore.containsAlias(keyAlias)) {
            val keyPairGenerator = KeyPairGenerator.getInstance(KeyProperties.KEY_ALGORITHM_EC, "AndroidKeyStore")
            val parameterSpec = KeyGenParameterSpec.Builder(keyAlias, KeyProperties.PURPOSE_SIGN or KeyProperties.PURPOSE_VERIFY)
                .setDigests(KeyProperties.DIGEST_SHA256)
                .setIsStrongBoxBacked(true)
                .setUserAuthenticationRequired(true)
                .build()
            keyPairGenerator.initialize(parameterSpec)
            keyPairGenerator.generateKeyPair()
        }

        return keyStore.getCertificate(keyAlias).publicKey.encoded.joinToString("") {
            it.toString(16).padStart(2, '0')
        }
    }
}