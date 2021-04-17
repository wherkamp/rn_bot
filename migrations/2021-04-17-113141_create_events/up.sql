CREATE TABLE events
(
    id          BIGINT AUTO_INCREMENT PRIMARY KEY,
    name        TEXT,
    description TEXT,
    creator     TEXT,
    active      bool DEFAULT false,
    end         BIGINT NULL,
    created     BIGINT
)