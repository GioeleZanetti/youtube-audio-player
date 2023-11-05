-- Your SQL goes here
CREATE TABLE song(
  id VARCHAR(50) PRIMARY KEY,
  name VARCHAR(200) NOT NULL
);

CREATE TABLE playlist(
  name VARCHAR(200) PRIMARY KEY
);

CREATE TABLE playlist_song(
  playlist_name VARCHAR(200) NOT NULL,
  song_id VARCHAR(50) NOT NULL,
  FOREIGN KEY(song_id) REFERENCES song(id) ON DELETE CASCADE ON UPDATE CASCADE,
  FOREIGN KEY(playlist_name) REFERENCES playlist(name) ON DELETE CASCADE ON UPDATE CASCADE,
  PRIMARY KEY(playlist_name, song_id)
);


