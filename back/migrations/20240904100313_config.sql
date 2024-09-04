INSERT INTO Config (
  max_order_age,
  session_duration,
  stripe_publishable_key,
  stripe_secret_key,
  smtp_username,
  smtp_password
) VALUES (
  10 * 60, -- in seconds
  12 * 60 * 60, -- in seconds
  "pk_test_51PnTG3B4cjDoCwmcQOtoriGBiGkBRZvSgmmBSvdJlgI1twafweuMVVzh7pvz1hmlIvK6UOju0vN5XRrGVs4auvHp00Sagkco96",
  "sk_test_51PnTG3B4cjDoCwmcWU0xG3EgLqPTMCSb6zIJOSbrjHBh0NhEtwynua4O3kyHoZMV6Zr6Q4Zwq4seALYwGx8OyDNg000CuyQYEv",
  "lhavraispay@gmail.com",
  "hbzg wnob wnac mxol"
);


INSERT INTO Bar (enforce_one_row) VALUES ("1"); -- populate w/ default values
