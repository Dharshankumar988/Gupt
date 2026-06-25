-- 007_create_avatars_bucket.sql
-- Creates the avatars bucket and adds RLS policies

-- Create "avatars" bucket if it doesn't exist
INSERT INTO storage.buckets (id, name, public) 
VALUES ('avatars', 'avatars', true)
ON CONFLICT (id) DO NOTHING;

-- Enable RLS
-- (Already enabled by default in Supabase storage schema)

-- Allow anon to upload to avatars bucket
CREATE POLICY "Avatar uploads" 
ON storage.objects 
FOR INSERT TO anon 
WITH CHECK (bucket_id = 'avatars');

-- Allow anon to select from avatars bucket
CREATE POLICY "Avatar views" 
ON storage.objects 
FOR SELECT TO anon 
USING (bucket_id = 'avatars');

-- Allow anon to update avatars
CREATE POLICY "Avatar updates" 
ON storage.objects 
FOR UPDATE TO anon 
USING (bucket_id = 'avatars');

-- Allow anon to delete avatars
CREATE POLICY "Avatar deletes" 
ON storage.objects 
FOR DELETE TO anon 
USING (bucket_id = 'avatars');
