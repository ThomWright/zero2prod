CREATE TABLE deliveries (
  id uuid NOT NULL PRIMARY KEY,
  newsletter_id uuid NOT NULL REFERENCES newsletters(newsletter_id),
  subscription_id uuid NOT NULL REFERENCES subscriptions(id)
);
