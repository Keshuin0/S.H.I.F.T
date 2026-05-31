@file:Suppress("DEPRECATION")

plugins {
    alias(libs.plugins.android.application)
}

android {
    namespace = "com.shift.core"

    // ⚡ UNLOCKING AVF API (Android 15)
    compileSdk = 37

    sourceSets {
        getByName("main") {
            jniLibs.srcDir("src/main/jniLibs")
        }
    }

    defaultConfig {
        applicationId = "com.shift.core"
        // ⚡ AVF requires a minimum of Android 14 (API 34)
        minSdk = 35
        targetSdk = 37
        versionCode = 1
        versionName = "1.0"

        testInstrumentationRunner = "androidx.test.runner.AndroidJUnitRunner"
    }

    buildTypes {
        release {
            isMinifyEnabled = false
            proguardFiles(
                getDefaultProguardFile("proguard-android-optimize.txt"),
                "proguard-rules.pro"
            )
        }
    }
    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_11
        targetCompatibility = JavaVersion.VERSION_11
    }
    buildFeatures {
        viewBinding = true
    }
    packaging {
        jniLibs {
            useLegacyPackaging = true
        }
    }
}

dependencies {
    implementation(libs.androidx.appcompat)
    implementation(libs.androidx.constraintlayout)
    implementation(libs.androidx.core.ktx)
    implementation(libs.material)
    testImplementation(libs.junit)
    androidTestImplementation(libs.androidx.espresso.core)
    androidTestImplementation(libs.androidx.junit)
}

tasks.register("compileRustCore") {
    group = "build"
    description = "Compiles the Rust shift_core binary and synchronizes the binaries to jniLibs."

    doLast {
        // Find if cargo and cargo-ndk are available
        val cargoCheck = try {
            val checkProc = ProcessBuilder("cargo", "--version").start()
            checkProc.waitFor() == 0
        } catch (e: Exception) {
            false
        }

        val ndkCheck = try {
            val checkProc = ProcessBuilder("cargo", "ndk", "--version").start()
            checkProc.waitFor() == 0
        } catch (e: Exception) {
            false
        }

        val jniLibsDir = file("src/main/jniLibs/arm64-v8a")
        if (!jniLibsDir.exists()) {
            jniLibsDir.mkdirs()
        }

        if (cargoCheck && ndkCheck) {
            println("⚙️ [CARGO NDK] Cargo and Cargo NDK detected. Starting automated compilation...")
            
            val rustProjectDir = File(project.rootDir.parentFile, "shift_core")
            
            // Execute cargo ndk compilation targeting Android platform API 35
            val cargoCmd = listOf("cargo", "ndk", "-t", "arm64-v8a", "-P", "35", "build", "--release")
            println("⚙️ [CARGO NDK] Executing: ${cargoCmd.joinToString(" ")}")
            
            val buildProc = ProcessBuilder(cargoCmd)
                .directory(rustProjectDir)
                .start()
            
            // Read output streams in separate threads to prevent deadlocks and capture logs
            val outputReader = Thread {
                buildProc.inputStream.bufferedReader().forEachLine { println(it) }
            }
            val errorReader = Thread {
                buildProc.errorStream.bufferedReader().forEachLine { System.err.println(it) }
            }
            outputReader.start()
            errorReader.start()
            
            val exitCode = buildProc.waitFor()
            outputReader.join()
            errorReader.join()
            
            if (exitCode != 0) {
                throw GradleException("❌ [CARGO NDK] Rust compilation failed with exit code $exitCode")
            }

            println("⚙️ [CARGO NDK] Rust compilation successful. Copying target assets...")

            // Copy shift_vault binary renamed as libshift_core.so
            val rustOutDir = File(rustProjectDir, "target/aarch64-linux-android/release")
            val shiftVaultExe = File(rustOutDir, "shift_vault")
            if (shiftVaultExe.exists()) {
                val targetCore = File(jniLibsDir, "libshift_core.so")
                shiftVaultExe.copyTo(targetCore, overwrite = true)
                println("⚙️ [CARGO NDK] Synchronized: ${targetCore.name}")
            } else {
                throw GradleException("❌ [CARGO NDK] Compiled shift_vault executable not found at ${shiftVaultExe.absolutePath}")
            }

            // Copy newly compiled libif_watch-*.so, cleaning up any stale ones
            val depsDir = File(rustOutDir, "deps")
            val newWatchLib = depsDir.listFiles()?.firstOrNull { it.name.startsWith("libif_watch-") && it.name.endsWith(".so") }
            if (newWatchLib != null) {
                // Clear any existing libif_watch-*.so first
                jniLibsDir.listFiles()?.forEach { 
                    if (it.name.startsWith("libif_watch-") && it.name.endsWith(".so")) {
                        it.delete()
                    }
                }
                val targetWatch = File(jniLibsDir, newWatchLib.name)
                newWatchLib.copyTo(targetWatch, overwrite = true)
                println("⚙️ [CARGO NDK] Synchronized: ${targetWatch.name}")
            } else {
                println("⚠️ [CARGO NDK] libif_watch-*.so not found in release deps. It may be statically linked or not generated yet.")
            }
        } else {
            println("⚠️ [CARGO NDK] cargo or cargo-ndk not found on PATH. Skipping compilation and using existing jniLibs assets.")
            val expectedCore = File(jniLibsDir, "libshift_core.so")
            if (!expectedCore.exists()) {
                println("❌ [CARGO NDK] ERROR: libshift_core.so missing from jniLibs and no compiler is available. App will fail on launch.")
            }
        }
    }
}

tasks.named("preBuild") {
    dependsOn("compileRustCore")
}