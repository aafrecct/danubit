CREATE TABLE IF NOT EXISTS users (
  id                UUID            DEFAULT gen_random_uuid()   PRIMARY KEY,
  username          VARCHAR(32)     UNIQUE NOT NULL,
  name              VARCHAR(32)     NOT NULL,
  surname           VARCHAR(64)     NOT NULL,
  email             EMAIL           UNIQUE NOT NULL,
  activated         BOOLEAN         NOT NULL,
  password_hash     CHAR(60),
  additional_info   JSONB           NOT NULL DEFAULT '{}'::jsonb
);
  
