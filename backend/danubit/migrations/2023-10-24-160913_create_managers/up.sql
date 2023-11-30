CREATE TABLE IF NOT EXISTS managers (
  id                      BIGINT          GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  user_id                 UUID            UNIQUE NOT NULL REFERENCES users,
  name                    VARCHAR(64)     UNIQUE NOT NULL,
  contact_email           EMAIL           NOT NULL,
  admin_email             EMAIL,
  material_email          EMAIL,
  print_email             EMAIL,
  comms_email             EMAIL
);

ALTER TABLE asociations
  ADD COLUMN manager      BIGINT          /* NOT NULL */ REFERENCES managers;
