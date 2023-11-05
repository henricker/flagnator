CREATE TABLE projects (
    id TEXT PRIMARY KEY,
    organization_id TEXT NOT NULL,
    name VARCHAR NOT NULL,
    CONSTRAINT fk_organization FOREIGN KEY (organization_id) REFERENCES organizations (id)
);