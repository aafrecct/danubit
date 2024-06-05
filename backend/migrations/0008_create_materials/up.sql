CREATE TABLE IF NOT EXISTS materials (
  id                      BIGINT          GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  asociation              UUID            NOT NULL REFERENCES asociations,
  name                    VARCHAR(64)     NOT NULL,
  description             TEXT            NOT NULL,
  quantity                SMALLINT        NOT NULL,
  available               SMALLINT        NOT NULL,
  is_lendable             BOOLEAN         NOT NULL,
  UNIQUE(asociation, name)
)
