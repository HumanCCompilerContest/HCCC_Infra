CREATE TYPE JudgeResult AS ENUM (
    'AC',
    'WA',
    'WC',
    'AE',
    'LE',
    'RE',
    'TLE',
    'Pending',
    'SystemError'
);

CREATE TYPE Arch AS ENUM (
    'x8664',
    'riscv'
);

CREATE TYPE TestTarget AS ENUM (
    'ExitCode',
    'StdOut',
    'NoTestCase'
);

CREATE TABLE accounts -- ユーザ
(
    id serial primary key,
    name text unique not null,
    password text not null
);

CREATE TABLE sessions
(
    session_key text primary key,
    user_id integer REFERENCES accounts(id) ON UPDATE NO ACTION ON DELETE CASCADE,
    created_at timestamptz not null
);

CREATE TABLE problems -- 問題
(
    id serial primary key,
    title text not null,
    statement text not null,
    code text not null,
    input_desc text,
    output_desc text,
    arch Arch,
    test_target TestTarget,
    is_wrong_code bool,
    error_line_number integer,
    score integer not null
);

CREATE TABLE testcases -- テストケース
(
    id serial primary key,
    problem_id integer REFERENCES problems(id) ON UPDATE NO ACTION ON DELETE CASCADE,
    input text,
    expect text
);

CREATE TABLE submits -- submit
(
    id serial primary key,
    user_id integer REFERENCES accounts(id) ON UPDATE NO ACTION ON DELETE CASCADE,
    problem_id integer REFERENCES problems(id) ON UPDATE NO ACTION ON DELETE CASCADE,
    time timestamptz not null,
    asm text not null,
    error_message text not null,
    is_ce boolean not null,
    error_line_number integer,
    result JudgeResult not null
);

INSERT INTO problems (id, title, statement, code, input_desc, output_desc, arch, test_target, is_wrong_code, error_line_number, score) VALUES (
    0,
    'Return 42',
    '42を返すプログラムを作成してください．',
    E'int main(void) {\nreturn 42;\n}',
    '無し',
    '無し',
    'x8664',
    'ExitCode',
    false,
    null,
    100
);

INSERT INTO testcases (problem_id, input, expect) VALUES(
    0,
    '',
    '42'
);

INSERT INTO problems (id, title, statement, code, input_desc, output_desc, arch, test_target, is_wrong_code, error_line_number, score) VALUES (
    1,
    'オウム返し',
    '数値をオウム返しするコードをコンパイルしてください．',
    E'int main(void) {\n\tint d;\n\tscanf("%d", &d);\n\tprintf("%d\\n", d);\n}',
    '1 <= n <= 10000',
    '入力と同じ値',
    'x8664',
    'ExitCode',
    false,
    null,
    100
);

INSERT INTO testcases (problem_id, input, expect) VALUES(
    1,
    '42',
    '42'
);

INSERT INTO testcases (problem_id, input, expect) VALUES(
    1,
    '37',
    '37'
);

INSERT INTO problems (id, title, statement, code, input_desc, output_desc, arch, test_target, is_wrong_code, error_line_number, score) VALUES (
    2,
    'return else',
    '以下のプログラムをコードの通りにコンパイルしてください.',
    E'int main(void) {\n\treturn else;\n}',
    '無し',
    'exitcodeで出力',
    'x8664',
    'NoTestCase',
    true,
    2,
    100
);

INSERT INTO testcases (problem_id, input, expect) VALUES(
    2,
    null,
    null
);
