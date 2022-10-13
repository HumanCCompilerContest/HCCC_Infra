CREATE TABLE tweets
(
    id        serial primary key,
    message   text        not null,
    posted_at timestamptz not null
);
