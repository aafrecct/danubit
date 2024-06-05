CREATE TABLE IF NOT EXISTS registration (
  id                      BIGINT          GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  activity                BIGINT          NOT NULL REFERENCES activities,
  user_id                 UUID            REFERENCES users,
  registration_data       JSONB           NOT NULL DEFAULT '{}'::jsonb,
  UNIQUE (activity, user_id)
)

