-- Add up migration script here

--- user table
CREATE TABLE users (
  id         BIGSERIAL NOT NULL ,
  mail_address VARCHAR(255) NOT NULL,
  password VARCHAR(255) NOT NULL,
  name       VARCHAR(255) NOT NULL,
  admin_flag smallint default 0,
  created_at TIMESTAMP NOT NULL default CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL default CURRENT_TIMESTAMP,
  PRIMARY KEY(id,mail_address)
);

--- group table 
CREATE TABLE groups (
  app_id    bigint NOT NULL,
  owner_id   bigint NOT NULL,
  name       VARCHAR(255) NOT NULL,
  created_at TIMESTAMP NOT NULL default CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL default CURRENT_TIMESTAMP,
  PRIMARY KEY(app_id, owner_id)
);

--- who is member of the group tbale 
CREATE TABLE group_menbers (
  group_id    bigint NOT NULL,
  member_id   bigint NOT NULL,
  admin_flag  smallint NOT NULL default 0,
  created_at TIMESTAMP NOT NULL default CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL default CURRENT_TIMESTAMP,
  PRIMARY KEY(group_id, member_id)
);

--- application table
CREATE TABLE applications (
  id    BIGSERIAL NOT NULL, 
  group_id bigint NOT NULL,
  name   VARCHAR(255) NOT NULL,
  created_at TIMESTAMP NOT NULL default CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL default CURRENT_TIMESTAMP,
  PRIMARY KEY(id)
);

--- page table
--- delete flag = 1 and now > expired at -> delete data 
CREATE TABLE web_pages (
    app_id bigint NOT NULL,
    page_path VARCHAR(512) NOT NULL,
    file_path VARCHAR(512) NOT NULL,
    delete_flag int default 0,
    expired_at TIMESTAMP,
    created_at TIMESTAMP NOT NULL default CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL default CURRENT_TIMESTAMP,
    PRIMARY KEY(app_id, page_path)
);

--- page_hierarchy table
--- ページの階層構造を保存しておく  
--- doc/index.md -> parent_path = doc, child_path = index.md
--- web_pages にinsert やupdate が来た時に変更される
CREATE TABLE page_hierarchy (
  id BIGSERIAL NOT NULL,
    app_id bigint NOT NULL,
    parent_path VARCHAR(512) NOT NULL NOT NULL,
    child_path VARCHAR(512) NOT NULL NOT NULL,
    depth int NOT NULL, --- child_path のdepth
    created_at TIMESTAMP NOT NULL default CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL default CURRENT_TIMESTAMP,
    PRIMARY KEY(id,app_id, parent_path, child_path)
);