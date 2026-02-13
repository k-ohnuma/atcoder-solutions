CREATE TABLE comments (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  user_id TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  solution_id uuid NOT NULL REFERENCES solutions(id) ON DELETE CASCADE,
  body_md TEXT NOT NULL,
  created_at TIMESTAMP(3) WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP(3),
  updated_at TIMESTAMP(3) WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP(3)
);

CREATE INDEX comments_solution_id_created_at_idx ON comments (solution_id, created_at DESC);
CREATE INDEX comments_user_id_created_at_idx ON comments (user_id, created_at DESC);

DROP TRIGGER IF EXISTS comments_updated_at_trigger ON comments;
CREATE TRIGGER comments_updated_at_trigger
  BEFORE UPDATE ON comments FOR EACH ROW
  EXECUTE PROCEDURE set_updated_at();
