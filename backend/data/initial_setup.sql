INSERT INTO contest_series(id, code, created_at)
VALUES
  (gen_random_uuid(), 'ABC', CURRENT_TIMESTAMP(3)),
  (gen_random_uuid(), 'ARC', CURRENT_TIMESTAMP(3)),
  (gen_random_uuid(), 'AGC', CURRENT_TIMESTAMP(3)),
  (gen_random_uuid(), 'AHC', CURRENT_TIMESTAMP(3)),
  (gen_random_uuid(), 'OTHER', CURRENT_TIMESTAMP(3))
ON CONFLICT(code) DO NOTHING;
