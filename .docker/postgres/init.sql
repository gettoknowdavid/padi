CREATE USER postgres WITH PASSWORD 'password';
CREATE DATABASE padi_dev OWNER postgres;
GRANT ALL PRIVILEGES ON DATABASE padi_dev TO postgres;