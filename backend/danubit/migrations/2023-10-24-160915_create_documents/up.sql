CREATE TABLE IF NOT EXISTS documents (
  id                      BIGINT          GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  asociation              UUID            NOT NULL REFERENCES asociations,
  activity                BIGINT          REFERENCES activities,
  name                    VARCHAR(64)     NOT NULL,
  description             TEXT            NOT NULL DEFAULT '',
  path                    VARCHAR(128)    NOT NULL,
  creation_date           DATE            NOT NULL,
  is_current              BOOLEAN         NOT NULL,
  is_important            BOOLEAN         NOT NULL,
  is_manager_accessible   BOOLEAN         NOT NULL,
  is_public_accessible    BOOLEAN         NOT NULL,
  UNIQUE(asociation, name)
)
