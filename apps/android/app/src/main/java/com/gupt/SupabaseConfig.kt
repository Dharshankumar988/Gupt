package com.gupt

import io.github.jan.supabase.createSupabaseClient
import io.github.jan.supabase.gotrue.Auth
import io.github.jan.supabase.postgrest.Postgrest
import io.github.jan.supabase.storage.Storage

object SupabaseConfig {
    // TODO: You MUST replace SUPABASE_KEY with your actual anon key from the Supabase Dashboard!
    private const val SUPABASE_URL = "https://phtecirdpzrxsnuhsapc.supabase.co"
    private const val SUPABASE_KEY = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZSIsInJlZiI6InBodGVjaXJkcHpyeHNudWhzYXBjIiwicm9sZSI6ImFub24iLCJpYXQiOjE3ODIzNjExNjksImV4cCI6MjA5NzkzNzE2OX0.soES-elTqhSpiElHQE3aADwV8EK80njMeHNKdtpJC98"

    val client = createSupabaseClient(
        supabaseUrl = SUPABASE_URL,
        supabaseKey = SUPABASE_KEY
    ) {
        install(Postgrest)
        install(Auth)
        install(Storage)
    }
}
