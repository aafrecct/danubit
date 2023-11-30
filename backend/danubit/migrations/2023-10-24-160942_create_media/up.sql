CREATE TYPE MEDIA_KIND AS ENUM ('logo', 'digital', 'print', 'screen', 'banner', 'extra');

CREATE TABLE IF NOT EXISTS media (
  id                      BIGINT          GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  name                    VARCHAR(32)     UNIQUE NOT NULL,
  activity                BIGINT          NOT NULL REFERENCES activities, 
  kind                    MEDIA_KIND      NOT NULL,
  path                    VARCHAR(128)    NOT NULL
);

ALTER TABLE asociations
  ADD COLUMN logo         BIGINT          REFERENCES media;
