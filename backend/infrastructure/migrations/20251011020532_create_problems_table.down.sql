DROP TRIGGER IF EXISTS problems_updated_at_trigger ON problems;
DROP TRIGGER IF EXISTS contests_updated_at_trigger ON contests;

DROP FUNCTION IF EXISTS set_updated_at;

DROP TABLE IF EXISTS problems;
DROP TABLE IF EXISTS contests;
DROP TABLE IF EXISTS contest_series;
