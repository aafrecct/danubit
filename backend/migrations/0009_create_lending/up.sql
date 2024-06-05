CREATE TABLE IF NOT EXISTS lendings (
  id                      BIGINT          GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  material                BIGINT          NOT NULL REFERENCES materials,
  user_id                 UUID            NOT NULL REFERENCES users,
  quantity                SMALLINT        NOT NULL,
  due_date                DATE            NOT NULL
);
