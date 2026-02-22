DELETE FROM roles r
WHERE r.name IN ('user', 'admin')
AND NOT EXISTS (
  SELECT 1 FROM users u WHERE u.role = r.name
);

DELETE FROM contest_series cs
WHERE cs.code IN ('ABC', 'ARC', 'AGC', 'AHC', 'OTHER')
AND NOT EXISTS (
  SELECT 1 FROM contests c WHERE c.series_code = cs.code
);
