ALTER TABLE contest_series
DROP CONSTRAINT IF EXISTS contest_series_code_check;

ALTER TABLE contest_series
ADD CONSTRAINT contest_series_code_check
CHECK (code IN ('ABC', 'ARC', 'AGC', 'AHC', 'AWC', 'OTHER'));

INSERT INTO contest_series (code)
VALUES ('AWC')
ON CONFLICT (code) DO NOTHING;
