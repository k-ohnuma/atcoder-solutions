CREATE TABLE solution_votes (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  user_id TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  solution_id uuid NOT NULL REFERENCES solutions(id) ON DELETE CASCADE,
  created_at TIMESTAMP(3) WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP(3),
  UNIQUE (user_id, solution_id)
);

CREATE INDEX solution_votes_solution_id_idx ON solution_votes (solution_id);
CREATE INDEX solution_votes_user_id_idx ON solution_votes (user_id);
