CREATE TABLE access_tokens (
  id SERIAL PRIMARY KEY,
  user_id SERIAL NOT NULL,
  expire_at TIMESTAMP NOT NULL,
  token VARCHAR NOT NULL,

  FOREIGN KEY (user_id) REFERENCES users(id)
)
