-- Add migration script here
CREATE TABLE Users (
  id uuid PRIMARY KEY,
  name VARCHAR(255) NOT NULL,
  email VARCHAR(255) NOT NULL UNIQUE,
  passhash VARCHAR(255) NOT NULL
);

CREATE TABLE Projects (
  id uuid PRIMARY KEY,
  name VARCHAR(255) NOT NULL,
  owner_id uuid REFERENCES Users(id) NOT NULL,
  CONSTRAINT qunique_name UNIQUE (name, owner_id)
);
CREATE TABLE ProjectsUsers (
  project_id uuid REFERENCES Projects(id) NOT NULL,
  shared_with uuid REFERENCES Users(id)
);

CREATE TABLE Tasks (
  id uuid PRIMARY KEY,
  title VARCHAR(255) NOT NULL,
  description VARCHAR(255) NOT NULL,
  project_id uuid REFERENCES Projects(id) NOT NULL,
  completed BOOLEAN DEFAULT FALSE NOT NULL
);