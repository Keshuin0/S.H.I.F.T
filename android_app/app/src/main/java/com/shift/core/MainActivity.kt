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
import java.security.KeyPairGenerator
import java.security.KeyStore

// The Rust Bridge
object TeeBridge {
    init { System.loadLibrary("shift_core") }
    external fun pingVault(command: String): String
    
    // NEW: Phase 3 - The Mathematical Rejection Engine (zk-PSI)
    external fun verifyProximityProof(scannedMacs: String, expectedMacs: String): String

    // NEW: Phase 4.1 - The zkVM
    external fun igniteZkVM(): String
}

class MainActivity : AppCompatActivity() {
    private lateinit var statusText: TextView

    // PHASE 1.5: The Mesh Memory Container
    private val nearbyNodes = mutableSetOf<String>()
    private var isMeshActive = false

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        val layout = LinearLayout(this)
        layout.orientation = LinearLayout.VERTICAL
        layout.setPadding(50, 50, 50, 50)

        statusText = TextView(this)
        statusText.textSize = 14f

        val polButton = Button(this)
        polButton.text = "EXECUTE: PHASE 1.5 (Proximity Mesh + PoL)"
        layout.addView(polButton)

        val lockButton = Button(this)
        lockButton.text = "EXECUTE: PHASE 2.4 (Fire Sub-50ms Lock)"
        layout.addView(lockButton)

        val genesisButton = Button(this)
        genesisButton.text = "EXECUTE: PHASE 3.1 (Mint Genesis Block)"
        layout.addView(genesisButton)

        // (Place this near your other button declarations)
        val zkvmButton = Button(this)
        zkvmButton.text = "EXECUTE: PHASE 4.1 (Ignite On-Device zkVM)"
        layout.addView(zkvmButton)

        // The Click Listener
        zkvmButton.setOnClickListener {
            statusText.append("\n\n[ALLOCATING MEMORY FOR NOVA IVC FOLDING...]")

            // Hit the Rust Core
            val vmResponse = TeeBridge.igniteZkVM()
            statusText.append("\n$vmResponse")
        }

        // NEW: Phase 3 - Mathematical Rejection Engine Trigger
        val zkPsiButton = Button(this)
        zkPsiButton.text = "EXECUTE: PHASE 3 (Test zk-PSI Rejection Engine)"
        layout.addView(zkPsiButton)

        layout.addView(statusText)
        setContentView(layout)

        // (Place this under your zkPsiButton setup)
        val pricingButton = Button(this)
        pricingButton.text = "EXECUTE: PHASE 4.3 (Hybrid Market-Maker & zkVM)"
        layout.addView(pricingButton)

        // 1. BOOT SEQUENCE
        try {
            val publicKeyHex = generateTrustZoneKey("SHIFT_SOVEREIGN_NODE_ID")
            val rustIdentityResponse = TeeBridge.pingVault("REGISTER_NODE:$publicKeyHex")

            val sbtToken = "SBT-CLEAR-ID-9942"
            val rustSbtResponse = TeeBridge.pingVault("ISSUE_SBT:$sbtToken")

            statusText.text = "SYSTEM BOOT:\n$rustIdentityResponse\n$rustSbtResponse\n\nReady for Telemetry."
        } catch (e: Exception) {
            statusText.text = "Boot Failed: ${e.message}"
        }

        // 2. TRIGGER PHASE 1.5
        polButton.setOnClickListener {
            if (checkAndRequestPermissions()) {
                if (!isMeshActive) {
                    startBleMesh()
                    statusText.append("\n\n[Activating BLE Mesh. Scanning Environment...]")
                }

                if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.P) {
                    triggerBiometricGate()
                }
            }
        }
        
        // ACTION: Phase 2.4 - The Rider's Strike
        lockButton.setOnClickListener {
            if (isMeshActive) {
                // HARDCODED DIAGNOSTIC SHARD: Target the specific Fold 6 Hexagon
                val targetZone = "zone:892b9ab93c7ffff"
                statusText.append("\n\n[FIRING LAMPORT TICKET INTO SHARD: $targetZone...]")

                // Strike the Vault
                val lockResponse = TeeBridge.pingVault("FIRE_LOCK:$targetZone")
                statusText.append("\n$lockResponse")
            } else {
                statusText.append("\n\n--- EXECUTION DENIED ---\nMesh must be active (Phase 1.5) before firing locks.")
            }
        }

        // ACTION: Phase 3.1 - Mint the Genesis Block
        genesisButton.setOnClickListener {
            statusText.append("\n\n[ANCHORING NODE TO BLOCK-LATTICE...]")
            val genesisResponse = TeeBridge.pingVault("MINT_GENESIS:")
            statusText.append("\n$genesisResponse")
        }

        // NEW ACTION: Phase 3 - Test the Mathematical Rejection Engine
        zkPsiButton.setOnClickListener {
            if (!isMeshActive) {
                statusText.append("\n\n--- ENGINE ERROR ---\nYou must activate the BLE Mesh (Phase 1.5) first to scan ambient MAC addresses.")
                return@setOnClickListener
            }

            statusText.append("\n\n[FIRING ZK-PSI MATHEMATICAL REJECTION ENGINE...]")

            // 1. What the phone physically scanned (from the BLE Scanner)
            val scannedString = if (nearbyNodes.isNotEmpty()) {
                nearbyNodes.joinToString(",")
            } else {
                "00:11:22:33:44:55"
            }

            // 2. Simulate what the DHT Network Expects in this H3 Hexagon
            // We will dynamically pull a few real MACs from your environment to simulate
            // a successful match, plus one fake one to simulate network reality.
            val expectedString = if (nearbyNodes.size >= 3) {
                val realMacsToMatch = nearbyNodes.take(3).joinToString(",")
                "$realMacsToMatch,FF:EE:DD:CC:BB:AA" // 3 real matches + 1 fake
            } else {
                // Fallback if your Bluetooth doesn't see at least 3 devices
                "00:11:22:33:44:55,AA:BB:CC:DD:EE:FF,11:22:33:44:55:66"
            }

            // 3. Hit the Rust Core
            val psiResult = TeeBridge.verifyProximityProof(scannedString, expectedString)

            statusText.append("\n$psiResult")
        }

        
    }

    // PHASE 1.5: Modern Android Permission Gate
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

    // PHASE 1.5: The Active Proximity Mesh Logic
    private fun startBleMesh() {
        val bluetoothManager = getSystemService(Context.BLUETOOTH_SERVICE) as BluetoothManager
        val adapter = bluetoothManager.adapter ?: return

        val scanner = adapter.bluetoothLeScanner
        val advertiser = adapter.bluetoothLeAdvertiser

        // Start Broadcasting our Node Presence
        val settings = AdvertiseSettings.Builder().setAdvertiseMode(AdvertiseSettings.ADVERTISE_MODE_LOW_LATENCY).build()
        val data = AdvertiseData.Builder().setIncludeDeviceName(false).build() // Anonymous ping
        advertiser?.startAdvertising(settings, data, object : AdvertiseCallback() {})

        // Start Sweeping the Environment
        val scanSettings = ScanSettings.Builder().setScanMode(ScanSettings.SCAN_MODE_LOW_LATENCY).build()
        scanner?.startScan(null, scanSettings, object : ScanCallback() {
            override fun onScanResult(callbackType: Int, result: ScanResult) {
                // PHASE 1.5: Record the raw MAC addresses of nearby BLE devices
                result.device?.address?.let { nearbyNodes.add(it) }
            }
        })

        isMeshActive = true
    }

    // ACTION: The OS-Level Biometric Prompt
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
        // 1. Grab GPS
        val locationManager = getSystemService(Context.LOCATION_SERVICE) as LocationManager
        val location: Location? = try {
            locationManager.getLastKnownLocation(LocationManager.GPS_PROVIDER) ?: locationManager.getLastKnownLocation(LocationManager.NETWORK_PROVIDER)
        } catch (e: SecurityException) { null }

        // 2. Format the BLE Mesh Array (Phase 1.5 CRITICAL UPDATE)
        val meshConsensus = if (nearbyNodes.isNotEmpty()) {
            "BLE:${nearbyNodes.joinToString(",")}"
        } else {
            "BLE:ISOLATED"
        }

        // 3. FUSE ALL DATA
        val telemetry = if (location != null) {
            "LAT:${location.latitude}|LON:${location.longitude}|ALT:${location.altitude}|$meshConsensus|TS:${System.currentTimeMillis()}"
        } else {
            "LAT:46.2382|LON:-63.1311|ALT:0.0|$meshConsensus|TS:${System.currentTimeMillis()}|[INDOOR_NO_FIX]"
        }

        // 4. Cryptographic Binding in Rust
        val polResponse = TeeBridge.pingVault("GENERATE_POL:$telemetry")
        statusText.append("\n$polResponse")
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