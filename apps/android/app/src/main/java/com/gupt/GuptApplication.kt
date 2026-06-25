package com.gupt

import android.app.Application
import android.util.Log
import dagger.hilt.android.HiltAndroidApp

@HiltAndroidApp
class GuptApplication : Application() {
    override fun onCreate() {
        super.onCreate()
        Log.i("GuptApplication", "Application started, loading Rust FFI bindings.")
        
        // System.loadLibrary is typically called by the UniFFI generated Kotlin code,
        // but we can ensure it's loaded here if needed.
        try {
            System.loadLibrary("gupt_ffi")
            Log.i("GuptApplication", "Successfully loaded libgupt_ffi.so")
        } catch (e: UnsatisfiedLinkError) {
            Log.e("GuptApplication", "Failed to load libgupt_ffi.so - Ensure cargo-ndk build was successful", e)
        }
    }
}
