CREATE TYPE ACTIVITY_ACCESS AS ENUM ('public', 'members', 'board');

CREATE TABLE IF NOT EXISTS activities (
  id                      BIGINT          GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  name                    VARCHAR(32)     NOT NULL,
  description             TEXT            NOT NULL,
  room                    VARCHAR(32)     NOT NULL,
  initial_date            TIMESTAMP       NOT NULL,
  duration                INT             NOT NULL,
  is_multi_session        BOOLEAN         NOT NULL,
  is_creditable           BOOLEAN         NOT NULL,
  is_external             BOOLEAN         NOT NULL,
  is_accepted             BOOLEAN         NOT NULL,
  is_room_accepted        BOOLEAN         NOT NULL,
  is_media_accepted       BOOLEAN         NOT NULL,
  is_registration_needed  BOOLEAN         NOT NULL,
  access                  ACTIVITY_ACCESS NOT NULL,
  additional_info         JSONB           NOT NULL DEFAULT '{}'::jsonb
);

CREATE TABLE IF NOT EXISTS organizers (
  id                      BIGINT          GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  asociation              UUID            NOT NULL REFERENCES asociations,
  activity                BIGINT          NOT NULL REFERENCES activities,
  person_in_charge        UUID            NOT NULL REFERENCES users,
  UNIQUE (asociation, activity)
);

