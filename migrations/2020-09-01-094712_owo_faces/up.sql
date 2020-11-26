-- Your SQL goes here
CREATE TABLE owo_faces (
    id int(11) NOT NULL AUTO_INCREMENT PRIMARY KEY,
    face varchar(255) NOT NULL,
    face_size int NOT NULL,
    style varchar(255) NOT NULL,
    emotion varchar(255) NOT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8;