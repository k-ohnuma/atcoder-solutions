INSERT INTO contest_series (code)
VALUES
  ('ABC'),
  ('ARC'),
  ('AGC'),
  ('AHC'),
  ('OTHER')
ON CONFLICT (code) DO NOTHING;

INSERT INTO roles (name)
VALUES
  ('user'),
  ('admin')
ON CONFLICT (name) DO NOTHING;
