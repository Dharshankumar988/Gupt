package com.gupt.di

import android.content.Context
import com.gupt.ffi.GuptEngine
import dagger.Module
import dagger.Provides
import dagger.hilt.InstallIn
import dagger.hilt.android.qualifiers.ApplicationContext
import dagger.hilt.components.SingletonComponent
import javax.inject.Singleton

@Module
@InstallIn(SingletonComponent::class)
object AppModule {

    @Provides
    @Singleton
    fun provideGuptEngine(@ApplicationContext context: Context): GuptEngine {
        // In a real app, the PIN and DB Path would be retrieved from encrypted SharedPreferences
        // or a secure Biometric prompt.
        val dbPath = context.getDatabasePath("gupt_secure.db").absolutePath
        val userPin = "user_secure_pin_from_keystore"
        
        return GuptEngine.initialize(dbPath, userPin)
    }
}
