CREATE TABLE newsletters (
   newsletter_id uuid NOT NULL PRIMARY KEY,
   title TEXT NOT NULL,
   text_content TEXT NOT NULL,
   html_content TEXT NOT NULL,
   published_at timestamptz NOT NULL
);