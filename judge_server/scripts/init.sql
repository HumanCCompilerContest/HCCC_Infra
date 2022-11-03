CREATE TYPE JudgeResult AS ENUM (
    'Pending',
    'AC',
    'WA',
    'AE',
    'LE',
    'TLE',
    'SystemError'
);

CREATE TABLE accounts ( -- ユーザ
	id serial primary key,
	name text unique not null,
	password text not null
    -- session data?
);

CREATE TABLE probrems ( -- 問題
	id serial primary key,
	title text not null,
	statement text not null,
	code text not null,
	input_desc text,
	output_desc text,
	score integer not null
);

CREATE TABLE submits ( -- submit
	id serial primary key,
	user_id serial not null,
	time timestamptz not null,
	asem text not null,
	result JudgeResult not null
);

CREATE TABLE testcases ( -- テストケース
	id serial primary key,
	sepecial_cond serial,
	input text not null,
	output text not null
);
