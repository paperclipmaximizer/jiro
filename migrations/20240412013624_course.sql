-- -----------------------------------------------------
-- Table course
-- -----------------------------------------------------
CREATE TABLE course(
  id uuid NOT NULL,
  PRIMARY KEY (id),
  name TEXT NOT NULL,
  user_id uuid NOT NULL,
  language TEXT NOT NULL,
  created_at timestamptz NOT NULL
);
