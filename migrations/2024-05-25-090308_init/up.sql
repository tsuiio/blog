CREATE TYPE publish_status AS ENUM ('public', 'draft', 'recycle');
CREATE TYPE page_type AS ENUM ('about');
CREATE TYPE sort_type AS ENUM ('note', 'page', 'url');

CREATE TABLE users (
   id UUID PRIMARY KEY,
   created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
   updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
   username VARCHAR(256) NOT NULL,
   nickname VARCHAR(256) NOT NULL,
   email VARCHAR(256) NOT NULL,
   password_hash VARCHAR(256) NOT NULL
);

CREATE TABLE short_ids(
   id UUID PRIMARY KEY,
   created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
   updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
   short_name VARCHAR(16) NOT NULL,
   subname VARCHAR(256)
);

CREATE TABLE notes(
   id UUID PRIMARY KEY,
   created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
   updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP, 
   title VARCHAR(256) NOT NULL,
   status publish_status NOT NULL,
   summary TEXT NOT NULL,
   content TEXT NOT NULL,
   views BIGINT NOT NULL,
   comm BOOLEAN NOT NULL,
   user_id UUID NOT NULL REFERENCES users(id),
   short_id UUID NOT NULL REFERENCES short_ids(id)
);

CREATE TABLE sorts(
   id UUID PRIMARY KEY,
   created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
   updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP, 
   name VARCHAR(256) NOT NULL,
   content VARCHAR(256) NOT NULL,
   sort_order INT NOT NULL,
   sort_type sort_type,
   parent_id UUID REFERENCES sorts(id)
);

CREATE TABLE note_sorts (
   PRIMARY KEY (note_id, sort_id),
   created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
   updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
   note_id UUID NOT NULL REFERENCES notes(id),
   sort_id UUID NOT NULL REFERENCES sorts(id)
);

CREATE TABLE pages(
   id UUID PRIMARY KEY,
   created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
   updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
   page_type page_type NOT NULL,
   status publish_status NOT NULL,
   comm BOOLEAN NOT NULL,
   user_id UUID NOT NULL REFERENCES users(id),
   short_id UUID NOT NULL REFERENCES short_ids(id)
);

CREATE TABLE page_sorts (
   PRIMARY KEY (page_id, sort_id),
   created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
   updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
   page_id UUID NOT NULL REFERENCES pages(id),
   sort_id UUID NOT NULL REFERENCES sorts(id)
);

CREATE TABLE page_about(
   id UUID PRIMARY KEY,
   avatar_url VARCHAR(2048) NOT NULL,
   content TEXT NOT NULL,
   page_id UUID NOT NULL REFERENCES pages(id)
);

CREATE TABLE tags(
   id UUID PRIMARY KEY,
   created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
   updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
   content VARCHAR(256) NOT NULL
);

CREATE TABLE note_tags (
   PRIMARY KEY (note_id, tag_id),
   created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
   updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
   note_id UUID NOT NULL REFERENCES notes(id),
   tag_id UUID NOT NULL REFERENCES tags(id)
);

CREATE TABLE comm_users(
   id UUID PRIMARY KEY,
   created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
   updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
   nickname VARCHAR(256) NOT NULL,
   email VARCHAR(256) NOT NULL,
   website_url VARCHAR(256)
);

CREATE TABLE comms(
   id UUID PRIMARY KEY,
   created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
   updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
   content TEXT NOT NULL,
   comm_user_id UUID REFERENCES comm_users(id),
   blog_user_id UUID REFERENCES users(id),
   note_id UUID REFERENCES notes(id),
   page_id UUID REFERENCES pages(id)
);

CREATE TABLE comms_closure (
   PRIMARY KEY (ancestor_id, descendant_id),
   created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
   updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
   distance BIGINT NOT NULL,
   ancestor_id UUID NOT NULL REFERENCES comms(id),
   descendant_id UUID NOT NULL REFERENCES comms(id)
);

CREATE TABLE info(
   id UUID PRIMARY KEY,
   created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
   updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
   bio TEXT NOT NULL,
   title VARCHAR(256) NOT NULL
);