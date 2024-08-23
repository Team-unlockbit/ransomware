drop table if exists users;
drop table if exists products;

CREATE TABLE users (
     id INT AUTO_INCREMENT PRIMARY KEY,
     username VARCHAR(50) NOT NULL,
     email VARCHAR(100) NOT NULL,
     created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE products (
     id INT AUTO_INCREMENT PRIMARY KEY,
     name VARCHAR(100) NOT NULL,
     description TEXT,
     price DECIMAL(10, 2) NOT NULL,
     created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

INSERT INTO users (username, email) VALUES ('alice', 'alice@example.com'), ('bob', 'bob@example.com'), ('charlie', 'charlie@example.com');
INSERT INTO products (name, description, price) VALUES ('Laptop', 'A high-performance laptop', 999.99), ('Smartphone', 'A latest model smartphone', 699.99), ('Headphones', 'Noise-cancelling over-ear headphones', 199.99);

select * from users;
select * from products;
