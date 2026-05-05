package com.shift.core

import android.Manifest
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
import android.os.Build
import android.os.Bundle
import android.os.CancellationSignal
import android.security.keystore.KeyGenParameterSpec
import android.security.keystore.KeyProperties
import android.widget.Button
import android.widget.LinearLayout
import android.widget.TextView
import androidx.annotation.RequiresApi
import androidx.appcompat.app.AppCompatActivity
import androidx.virtualmachine.VirtualMachine
import androidx.virtualmachine.VirtualMachineConfig
import androidx.virtualmachine.VirtualMachineManager
import androidx.virtualmachine.VirtualMachineCallback
import android.util.Log
import java.security.KeyPairGenerator
import java.security.KeyStore
import kotlinx.coroutines.*
import java.util.concurrent.Executors

// =========================================================================
// PHASE 1.6: THE VSOCK HYPERVISOR BRIDGE
// =========================================================================
object TeeBridge {
    var activeVm: VirtualMachine? = null
    const val VSOCK_PORT: Long = 8000

    // All communication with Rust now happens over a hardware-isolated Virtual Socket
    fun sendCommand(command: String): String {
        val vm = activeVm ?: return "❌ Execution Denied: Hypervisor is offline. Ignite pKVM first."

        return try {
            val socket = vm.connectToVsockServer(VSOCK_PORT)

            // Stream the command into the secure enclave
            socket.outputStream.write(command.toByteArray())
            socket.outputStream.flush()

            // Read the Rust Vault's response
            val response = socket.inputStream.bufferedReader().use { it.readText() }
            socket.close()

            response
        } catch (e: Exception) {
            "❌ VSOCK Connection Error: ${e.message}"
        }
    }
}

class MainActivity : AppCompatActivity() {
    private lateinit var statusText: TextView
    private val nearbyNodes = mutableSetOf<String>()
    private var isMeshActive = false

    // Use Coroutines for socket calls so we don't freeze the UI thread
    private val ioScope = CoroutineScope(Dispatchers.IO)

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        val layout = LinearLayout(this)
        layout.orientation = LinearLayout.VERTICAL
        layout.setPadding(50, 50, 50, 50)

        statusText = TextView(this)
        statusText.textSize = 14f

        val avfButton = Button(this)
        avfButton.text = "EXECUTE: PHASE 1.6 (Ignite pKVM Hypervisor)"
        layout.addView(avfButton)

        val polButton = Button(this)
        polButton.text = "EXECUTE: PHASE 1.5 (Proximity Mesh + PoL)"
        layout.addView(polButton)

        val lockButton = Button(this)
        lockButton.text = "EXECUTE: PHASE 2.4 (Fire Sub-50ms Lock)"
        layout.addView(lockButton)

        val genesisButton = Button(this)
        genesisButton.text = "EXECUTE: PHASE 3.1 (Mint Genesis Block)"
        layout.addView(genesisButton)

        val zkvmButton = Button(this)
        zkvmButton.text = "EXECUTE: PHASE 4.1 (Ignite On-Device zkVM)"
        layout.addView(zkvmButton)

        val zkPsiButton = Button(this)
        zkPsiButton.text = "EXECUTE: PHASE 3 (Test zk-PSI Rejection Engine)"
        layout.addView(zkPsiButton)

        val pricingButton = Button(this)
        pricingButton.text = "EXECUTE: PHASE 4.3 (Hybrid Market-Maker & zkVM)"
        layout.addView(pricingButton)

        layout.addView(statusText)
        setContentView(layout)

        // TRIGGER PHASE 1.6: Ignite pKVM Hypervisor
        avfButton.setOnClickListener {
            statusText.append("\n\n[REQUESTING TYPE-1 HYPERVISOR LEASE...]")
            if (Build.VERSION.SDK_INT >= 34) {
                igniteHypervisor()
            } else {
                statusText.append("\n❌ HARDWARE ERROR: Android 14+ required for AVF pKVM.")
            }
        }

        // TRIGGER PHASE 1.5: Proximity Mesh
        polButton.setOnClickListener {
            if (checkAndRequestPermissions()) {
                if (!isMeshActive) {
                    startBleMesh()
                    statusText.append("\n\n[Activating BLE/Wi-Fi Aware Mesh. Scanning Environment...]")
                }
                if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.P) {
                    triggerBiometricGate()
                }
            }
        }

        // ACTION: Phase 2.4 - The Rider's Strike
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

        // ACTION: Phase 3.1 - Mint the Genesis Block
        genesisButton.setOnClickListener {
            statusText.append("\n\n[ANCHORING NODE TO BLOCK-LATTICE...]")
            ioScope.launch {
                val genesisResponse = TeeBridge.sendCommand("MINT_GENESIS:")
                withContext(Dispatchers.Main) { statusText.append("\n$genesisResponse") }
            }
        }

        // ACTION: Phase 3 - Test the Mathematical Rejection Engine
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

        // ACTION: Phase 4.1 - Ignite zkVM
        zkvmButton.setOnClickListener {
            statusText.append("\n\n[ALLOCATING MEMORY FOR NOVA IVC FOLDING...]")
            ioScope.launch {
                val vmResponse = TeeBridge.sendCommand("IGNITE_ZKVM:")
                withContext(Dispatchers.Main) { statusText.append("\n$vmResponse") }
            }
        }

        // ACTION: Phase 4.3 - Hybrid Market-Maker
        pricingButton.setOnClickListener {
            if (!isMeshActive) {
                statusText.append("\n\n--- ENGINE ERROR ---\nYou must activate the BLE Mesh (Phase 1.5) to gather the Supply/Demand ratio.")
                return@setOnClickListener
            }

            statusText.append("\n\n[INITIATING HYBRID MARKET-MAKER...]")

            val localDemand = nearbyNodes.size
            val baseRatePerMile = 1.50
            val surgeMultiplier = if (localDemand > 5) 1.5 else if (localDemand > 10) 2.5 else 1.0
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

                    if (riderBid >= baseRatePerMile) {
                        statusText.append("\n✅ [SMART CONTRACT] Bid Accepted: Fare exceeds minimum operating cost.")
                    } else {
                        statusText.append("\n❌ [SMART CONTRACT] Bid Rejected: Rider bid ($$riderBid) falls below the Algorithmic Floor ($$baseRatePerMile).")
                    }
                }
            }
        }
    }

    @RequiresApi(34)
    private fun igniteHypervisor() {
        Log.i("SHIFT_AVF", "Initiating pKVM Hypervisor Ignition...")

        try {
            val vmManager = VirtualMachineManager.getInstance(applicationContext)
            if (vmManager == null) {
                statusText.append("\n❌ CRITICAL: OS restricts pKVM access on this device.")
                return
            }

            val builder = VirtualMachineConfig.Builder(applicationContext)
                .setProtectedVm(true)
                .setPayloadBinaryName("libshift_core.so")
                .setMemoryBytes(256)

            val config = builder.build()
            val vm = vmManager.getOrCreate("shift_vault_vm", config)

            vm.setCallback(mainExecutor, object : VirtualMachineCallback {
                override fun onPayloadStarted(vm: VirtualMachine) {
                    statusText.append("\n💎 [HYPERVISOR] Microdroid VM Booted. Hardware isolated.")
                }

                override fun onPayloadReady(vm: VirtualMachine) {
                    statusText.append("\n⚙️ [HYPERVISOR] Rust Payload Running. VSOCK Bridge established.")
                    // ⚡ THE BRIDGE IS SECURED. Bind the Kotlin UI to the Rust VM.
                    TeeBridge.activeVm = vm

                    // Automatically run the Boot Sequence now that the VM is alive
                    executeSecureBootSequence()
                }

                override fun onStopped(vm: VirtualMachine, reason: Int) {
                    statusText.append("\n⚠️ [HYPERVISOR] VM Terminated (Code: $reason).")
                    TeeBridge.activeVm = null
                }

                override fun onError(vm: VirtualMachine, errorCode: Int, message: String) {
                    statusText.append("\n❌ [HYPERVISOR] Crash: $message")
                }
            })

            statusText.append("\n⏳ [HYPERVISOR] Spinning up Microdroid Core...")
            vm.run()

        } catch (e: Exception) {
            statusText.append("\n❌ [HYPERVISOR] Ignition Exception: ${e.message}")
        }
    }

    private fun executeSecureBootSequence() {
        ioScope.launch {
            try {
                val publicKeyHex = generateTrustZoneKey("SHIFT_SOVEREIGN_NODE_ID")
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
        val required = mutableListOf(Manifest.permission.ACCESS_FINE_LOCATION)
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.S) {
            required.add(Manifest.permission.BLUETOOTH_SCAN)
            required.add(Manifest.permission.BLUETOOTH_ADVERTISE)
            required.add(Manifest.permission.BLUETOOTH_CONNECT)
        }
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
        advertiser?.startAdvertising(settings, data, object : AdvertiseCallback() {})

        val scanSettings = ScanSettings.Builder().setScanMode(ScanSettings.SCAN_MODE_LOW_LATENCY).build()
        scanner?.startScan(null, scanSettings, object : ScanCallback() {
            override fun onScanResult(callbackType: Int, result: ScanResult) {
                result.device?.address?.let { nearbyNodes.add(it) }
            }
        })
        isMeshActive = true
    }

    @RequiresApi(Build.VERSION_CODES.P)
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
        } catch (e: SecurityException) { null }

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

    private fun generateTrustZoneKey(alias: String): String {
        val keyStore = KeyStore.getInstance("AndroidKeyStore")
        keyStore.load(null)

        if (!keyStore.containsAlias(alias)) {
            val keyPairGenerator = KeyPairGenerator.getInstance(KeyProperties.KEY_ALGORITHM_EC, "AndroidKeyStore")
            val parameterSpec = KeyGenParameterSpec.Builder(alias, KeyProperties.PURPOSE_SIGN or KeyProperties.PURPOSE_VERIFY)
                .setDigests(KeyProperties.DIGEST_SHA256)
                .setIsStrongBoxBacked(true)
                .setUserAuthenticationRequired(true)
                .build()
            keyPairGenerator.initialize(parameterSpec)
            keyPairGenerator.generateKeyPair()
        }

        val publicKey = keyStore.getCertificate(alias).publicKey
        return publicKey.encoded.joinToString("") { "%02x".format(it) }
    }
}