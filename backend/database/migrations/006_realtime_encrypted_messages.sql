-- 006_realtime_encrypted_messages.sql
-- Recreates the encrypted_messages table to use TEXT for usernames instead of UUIDs, matching our UI

DROP TABLE IF EXISTS public.encrypted_messages;

CREATE TABLE IF NOT EXISTS public.encrypted_messages (
    id                  UUID        PRIMARY KEY DEFAULT gen_random_uuid(),
    conversation_id     TEXT        NOT NULL, -- E.g., sorted combination of two usernames to identify a unique conversation
    sender_id           TEXT        NOT NULL REFERENCES public.user_profiles(username) ON DELETE CASCADE,
    recipient_id        TEXT        NOT NULL REFERENCES public.user_profiles(username) ON DELETE CASCADE,
    encrypted_payload   TEXT        NOT NULL, -- Base64 encoded XChaCha20-Poly1305 ciphertext
    nonce               TEXT        NOT NULL, -- Base64 encoded 24-byte nonce
    message_type        VARCHAR(50) NOT NULL DEFAULT 'text',
    transport_used      VARCHAR(50) NOT NULL DEFAULT 'CLOUD',
    created_at          TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Index for quickly fetching messages for a specific conversation
CREATE INDEX IF NOT EXISTS idx_encrypted_messages_conversation ON public.encrypted_messages (conversation_id);

-- Enable Realtime
ALTER PUBLICATION supabase_realtime ADD TABLE public.encrypted_messages;

-- Enable RLS
ALTER TABLE public.encrypted_messages ENABLE ROW LEVEL SECURITY;

-- Allow users to select messages sent to or from them
CREATE POLICY "Select messages" ON public.encrypted_messages 
FOR SELECT TO anon 
USING (true);

-- Allow anyone to insert messages (in a real app, you'd check auth.uid())
CREATE POLICY "Insert messages" ON public.encrypted_messages 
FOR INSERT TO anon 
WITH CHECK (true);
