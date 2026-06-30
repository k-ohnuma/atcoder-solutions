ALTER TABLE problems
ADD CONSTRAINT problems_contest_index_uniq UNIQUE (contest_code, problem_index);
