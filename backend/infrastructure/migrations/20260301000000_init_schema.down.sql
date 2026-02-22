DROP TRIGGER IF EXISTS comments_updated_at_trigger ON comments;
DROP TRIGGER IF EXISTS solutions_updated_at_trigger ON solutions;
DROP TRIGGER IF EXISTS users_updated_at_trigger ON users;
DROP TRIGGER IF EXISTS problems_updated_at_trigger ON problems;
DROP TRIGGER IF EXISTS contests_updated_at_trigger ON contests;

DROP TABLE IF EXISTS comments;
DROP TABLE IF EXISTS solution_votes;
DROP TABLE IF EXISTS solution_tags;
DROP TABLE IF EXISTS tags;
DROP TABLE IF EXISTS solutions;
DROP TABLE IF EXISTS users;
DROP TABLE IF EXISTS roles;
DROP TABLE IF EXISTS problems;
DROP TABLE IF EXISTS contests;
DROP TABLE IF EXISTS contest_series;

DROP FUNCTION IF EXISTS set_updated_at();
