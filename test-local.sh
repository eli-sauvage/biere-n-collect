docker compose -f ../compose.test.yml up db-test -d
DATABASE_URL=mysql://root:test-root-password@localhost:1234/biere-n-collect sqlx migrate run
DATABASE_URL=mysql://root:test-root-password@localhost:1234/biere-n-collect cargo test
