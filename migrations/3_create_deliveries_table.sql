CREATE TABLE deliveries (
  id uuid NOT NULL PRIMARY KEY,
  newsletter_id uuid REFERENCES newsletters(newsletter_id),
  subscription_id uuid REFERENCES subscriptions(id)
);
