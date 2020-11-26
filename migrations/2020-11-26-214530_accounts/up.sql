-- Your SQL goes here
CREATE TABLE accounts (
    id int(11) NOT NULL AUTO_INCREMENT PRIMARY KEY,
    ip_addr varchar(255) NOT NULL,
    token varchar(255) NOT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8;