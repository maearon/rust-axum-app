-- Create microposts table
CREATE TABLE microposts (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    content TEXT NOT NULL,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    picture VARCHAR(255),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Create indexes
CREATE INDEX idx_microposts_user_id ON microposts(user_id);
CREATE INDEX idx_microposts_created_at ON microposts(created_at DESC);
CREATE INDEX idx_microposts_user_created ON microposts(user_id, created_at DESC);

-- Add constraint for content length
ALTER TABLE microposts ADD CONSTRAINT chk_content_length CHECK (char_length(content) <= 140);
