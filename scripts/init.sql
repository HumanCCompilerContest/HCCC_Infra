CREATE TYPE JudgeResult AS ENUM (
    'Pending',
    'AC',
    'WA',
    'AE',
    'LE',
    'TLE',
    'SystemError'
);

CREATE TABLE accounts -- ユーザ
(
	id serial primary key,
	name text unique not null,
	password text not null,
	score integer not null
);

CREATE TABLE sessions
(
	session_key text primary key,
	user_id integer not null,
	created_at timestamptz not null,
	FOREIGN KEY (user_id) REFERENCES accounts(id)
		ON UPDATE NO ACTION ON DELETE CASCADE
);

CREATE TABLE problems -- 問題
(
	id serial primary key,
	title text not null,
	statement text not null,
	code text not null,
	input_desc text,
	output_desc text,
	score integer not null
);

CREATE TABLE submits -- submit
(
	id serial primary key,
	user_id integer not null,
	problem_id integer not null,
	time timestamptz not null,
	asm text not null,
	result JudgeResult not null
	-- FOREIGN KEY (user_id, problem_id) REFERENCES accounts(id), problems(id)
	-- 	ON UPDATE NO ACTION ON DELETE CASCADE
);

INSERT INTO problems (id, title, statement, code, input_desc, output_desc, score) VALUES (
    0,
    'Return 42',
    '42を返すプログラムを作成してください．',
    E'int main(void) {\nreturn 42;\n}',
    '無し',
    '無し',
    100
);
