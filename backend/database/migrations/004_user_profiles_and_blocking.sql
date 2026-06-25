-- 004_user_profiles_and_blocking.sql
-- Adds user profiles (display name, avatar, relay rewards), blocked users, and relay log

-- =========================================================================
-- User Profiles
-- =========================================================================
CREATE TABLE IF NOT EXISTS public.user_profiles (
    username        TEXT PRIMARY KEY,
    display_name    TEXT NOT NULL DEFAULT '',
    avatar_url      TEXT,
    relay_rewards_inr DECIMAL(12, 4) NOT NULL DEFAULT 0.0000,
    is_online       BOOLEAN NOT NULL DEFAULT false,
    last_seen       TIMESTAMPTZ DEFAULT NOW()
);

ALTER TABLE public.user_profiles ENABLE ROW LEVEL SECURITY;

CREATE POLICY "anon_select_user_profiles" ON public.user_profiles FOR SELECT TO anon USING (true);
CREATE POLICY "anon_insert_user_profiles" ON public.user_profiles FOR INSERT TO anon WITH CHECK (true);
CREATE POLICY "anon_update_user_profiles" ON public.user_profiles FOR UPDATE TO anon USING (true);

-- =========================================================================
-- Blocked Users
-- =========================================================================
CREATE TABLE IF NOT EXISTS public.blocked_users (
    id              UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    blocker_username TEXT NOT NULL,
    blocked_username TEXT NOT NULL,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(blocker_username, blocked_username)
);

ALTER TABLE public.blocked_users ENABLE ROW LEVEL SECURITY;

CREATE POLICY "anon_select_blocked_users" ON public.blocked_users FOR SELECT TO anon USING (true);
CREATE POLICY "anon_insert_blocked_users" ON public.blocked_users FOR INSERT TO anon WITH CHECK (true);
CREATE POLICY "anon_delete_blocked_users" ON public.blocked_users FOR DELETE TO anon USING (true);

-- =========================================================================
-- Relay Log (tracks messages relayed by middlemen)
-- =========================================================================
CREATE TABLE IF NOT EXISTS public.relay_log (
    id              UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    relayer_username TEXT NOT NULL,
    message_hash    TEXT NOT NULL,
    reward_inr      DECIMAL(12, 4) NOT NULL DEFAULT 0.0001,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

ALTER TABLE public.relay_log ENABLE ROW LEVEL SECURITY;

CREATE POLICY "anon_select_relay_log" ON public.relay_log FOR SELECT TO anon USING (true);
CREATE POLICY "anon_insert_relay_log" ON public.relay_log FOR INSERT TO anon WITH CHECK (true);
