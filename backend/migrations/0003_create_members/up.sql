CREATE TYPE BOARD_STATUS AS ENUM ('false', 'board', 'vice_chair', 'chair');

CREATE TABLE IF NOT EXISTS members (
  id                BIGINT          GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  user_id           UUID            NOT NULL REFERENCES users,
  asociation        UUID            NOT NULL REFERENCES asociations,
  is_accepted       BOOLEAN         NOT NULL,
  accepted_date     DATE,
  expiry_date       DATE,
  label             VARCHAR(32),
  board_status      BOARD_STATUS    NOT NULL,
  UNIQUE (user_id, asociation)
);
