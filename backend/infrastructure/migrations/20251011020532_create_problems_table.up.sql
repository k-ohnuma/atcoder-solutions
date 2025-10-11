-- initial-setupで格納
CREATE TABLE IF NOT EXISTS contest_series (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  code  TEXT NOT NULL UNIQUE CHECK (code IN ('ABC', 'ARC', 'AGC', 'AHC', 'OTHER')),
  created_at TIMESTAMP(3) WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP(3)
);

-- jsonから格納
CREATE TABLE IF NOT EXISTS contests (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  code TEXT NOT NULL UNIQUE,
  series_code TEXT NOT NULL REFERENCES contest_series(code) ON DELETE RESTRICT,
  created_at TIMESTAMP(3) WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP(3),
  updated_at TIMESTAMP(3) WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP(3)
);

CREATE TABLE IF NOT EXISTS problems (
  id TEXT PRIMARY KEY,
  contest_code TEXT NOT NULL REFERENCES contests(code) ON DELETE RESTRICT,
  problem_index TEXT NOT NULL,
  title TEXT NOT NULL,
  created_at TIMESTAMP(3) WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP(3),
  updated_at TIMESTAMP(3) WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP(3),
  CONSTRAINT problems_contest_index_uniq UNIQUE (contest_code, problem_index)
);

CREATE OR REPLACE FUNCTION set_updated_at()
RETURNS trigger AS $$
BEGIN
  NEW.updated_at := CURRENT_TIMESTAMP(3);
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS contests_updated_at_trigger ON contests;
CREATE TRIGGER contests_updated_at_trigger
  BEFORE UPDATE ON contests FOR EACH ROW
  EXECUTE PROCEDURE set_updated_at();

DROP TRIGGER IF EXISTS problems_updated_at_trigger ON problems;
CREATE TRIGGER problems_updated_at_trigger
  BEFORE UPDATE ON problems FOR EACH ROW
  EXECUTE PROCEDURE set_updated_at();
