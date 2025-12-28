-- Create profiles table
CREATE TABLE IF NOT EXISTS profiles (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    keyboard_vid INTEGER NOT NULL,
    keyboard_pid INTEGER NOT NULL,
    config_json TEXT NOT NULL,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL
);

-- Create keyboard_configs table to store last saved config per keyboard
CREATE TABLE IF NOT EXISTS keyboard_configs (
    keyboard_vid INTEGER NOT NULL,
    keyboard_pid INTEGER NOT NULL,
    config_json TEXT NOT NULL,
    updated_at INTEGER NOT NULL,
    PRIMARY KEY (keyboard_vid, keyboard_pid)
);

-- Create indexes
CREATE INDEX IF NOT EXISTS idx_profiles_keyboard ON profiles(keyboard_vid, keyboard_pid);
CREATE INDEX IF NOT EXISTS idx_profiles_updated ON profiles(updated_at);

