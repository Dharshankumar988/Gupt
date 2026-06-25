-- Create chat_backups table
CREATE TABLE public.chat_backups (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_username TEXT NOT NULL,
    payload JSONB NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT timezone('utc'::text, now()) NOT NULL
);

-- Set up Row Level Security (RLS)
ALTER TABLE public.chat_backups ENABLE ROW LEVEL SECURITY;

-- Allow all operations for now (can be restricted later)
CREATE POLICY "Allow anonymous read access on chat_backups" 
ON public.chat_backups FOR SELECT 
TO anon 
USING (true);

CREATE POLICY "Allow anonymous insert access on chat_backups" 
ON public.chat_backups FOR INSERT 
TO anon 
WITH CHECK (true);

CREATE POLICY "Allow anonymous update access on chat_backups" 
ON public.chat_backups FOR UPDATE 
TO anon 
USING (true);

CREATE POLICY "Allow anonymous delete access on chat_backups" 
ON public.chat_backups FOR DELETE 
TO anon 
USING (true);
