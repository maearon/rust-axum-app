-- Create relationships table
CREATE TABLE relationships (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    follower_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    followed_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Create indexes
CREATE INDEX idx_relationships_follower_id ON relationships(follower_id);
CREATE INDEX idx_relationships_followed_id ON relationships(followed_id);
CREATE INDEX idx_relationships_follower_followed ON relationships(follower_id, followed_id);

-- Add unique constraint to prevent duplicate relationships
ALTER TABLE relationships ADD CONSTRAINT unique_relationships UNIQUE (follower_id, followed_id);

-- Add constraint to prevent self-following
ALTER TABLE relationships ADD CONSTRAINT chk_no_self_follow CHECK (follower_id != followed_id);
