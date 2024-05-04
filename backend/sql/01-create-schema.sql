-- Task Type enum
CREATE TYPE task_type_enum AS ENUM (
  'work',
  'personal'
);

-- Task status enum
CREATE TYPE task_status_enum AS ENUM (
  'todo',
  'bug',
  'doing',
  'testing',
  'done'
);

-- Task Priority enum
CREATE TYPE task_priority_enum AS ENUM (
  'low',
  'medium',
  'high'
);

-- Task
CREATE TABLE task (
  id bigserial,
  cid bigint NOT NULL, -- creator user id
	ctime timestamp with time zone DEFAULT now(),
  mid bigint, -- modifier user id
	mtime timestamp with time zone,   
  title text NOT NULL,
  description text NOT NULL,
  due_date text NOT NULL,
  typ task_type_enum NOT NULL DEFAULT 'work',
  status task_status_enum NOT NULL DEFAULT 'todo',
  priority task_priority_enum NOT NULL DEFAULT 'low'
);
ALTER SEQUENCE task_id_seq RESTART WITH 1000;
