ALTER TABLE contest_series
DROP CONSTRAINT IF EXISTS contest_series_code_check;

DELETE FROM contest_series cs
WHERE cs.code = 'AWC'
AND NOT EXISTS (
  SELECT 1 FROM contests c WHERE c.series_code = cs.code
);

ALTER TABLE contest_series
ADD CONSTRAINT contest_series_code_check
CHECK (code IN ('ABC', 'ARC', 'AGC', 'AHC', 'OTHER'));
