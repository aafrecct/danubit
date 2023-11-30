CREATE DOMAIN email AS VARCHAR
  CHECK ( VALUE ~ '.*@.*' );

CREATE TABLE IF NOT EXISTS asociations (
  id                    UUID            DEFAULT gen_random_uuid()   PRIMARY KEY,
  short_name            VARCHAR(24)     UNIQUE NOT NULL,
  long_name             VARCHAR(128)    UNIQUE NOT NULL,
  email                 EMAIL           UNIQUE NOT NULL,
  description           TEXT            NOT NULL,
  is_public_joinable    BOOLEAN         NOT NULL,
  info                  JSONB           NOT NULL
);
  
