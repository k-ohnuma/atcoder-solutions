CREATE TABLE solutions (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  problem_id TEXT NOT NULL REFERENCES problems(id) ON DELETE RESTRICT,
  user_id TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  body_md text NOT NULL,
  submit_url text,
  created_at TIMESTAMP(3) WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP(3),
  updated_at TIMESTAMP(3) WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP(3)
);

CREATE TABLE tags (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  name TEXT NOT NULL UNIQUE,
  created_at TIMESTAMP(3) WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP(3)
);

CREATE TABLE solution_tags (
  solution_id uuid NOT NULL REFERENCES solutions(id) ON DELETE CASCADE,
  tag_id uuid NOT NULL REFERENCES tags(id) ON DELETE CASCADE,
  created_at TIMESTAMP(3) WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP(3),
  PRIMARY KEY (solution_id, tag_id)
);

CREATE INDEX solution_tags_tag_id_idx ON solution_tags (tag_id, solution_id);

DROP TRIGGER IF EXISTS solutions_updated_at_trigger ON solutions;
CREATE TRIGGER solutions_updated_at_trigger
  BEFORE UPDATE ON solutions FOR EACH ROW
  EXECUTE PROCEDURE set_updated_at();

