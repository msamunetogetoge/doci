-- Add up migration script here
CREATE TABLE users (
  id         BIGSERIAL NOT NULL ,
  mail_address VARCHAR(255) NOT NULL,
  name       VARCHAR(255) NOT NULL,
  admin_flag smallint default 0,
  created_at TIMESTAMP NOT NULL default CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL default CURRENT_TIMESTAMP,
  PRIMARY KEY(id,mail_address)
);

CREATE TABLE groups (
  app_id    bigint NOT NULL,
  owner_id   bigint NOT NULL,
  name       VARCHAR(255) NOT NULL,
  created_at TIMESTAMP NOT NULL default CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL default CURRENT_TIMESTAMP,
  PRIMARY KEY(app_id, owner_id)
);

CREATE TABLE group_menbers (
  group_id    bigint NOT NULL,
  member_id   bigint NOT NULL,
  admin_flag  smallint NOT NULL default 0,
  created_at TIMESTAMP NOT NULL default CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL default CURRENT_TIMESTAMP,
  PRIMARY KEY(group_id, member_id)
);

CREATE TABLE applications (
  id    BIGSERIAL NOT NULL, 
  group_id bigint NOT NULL,
  name   VARCHAR(255) NOT NULL,
  created_at TIMESTAMP NOT NULL default CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL default CURRENT_TIMESTAMP,
  PRIMARY KEY(id)
);

CREATE TABLE web_pages (
    app_id bigint NOT NULL,
    page_path VARCHAR(512) NOT NULL,
    body text NOT NULL,
    created_at TIMESTAMP NOT NULL default CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL default CURRENT_TIMESTAMP,
    PRIMARY KEY(app_id, page_path)
);