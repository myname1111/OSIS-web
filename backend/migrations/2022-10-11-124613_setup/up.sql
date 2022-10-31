CREATE TABLE images (
    id INTEGER PRIMARY KEY,
    i_path VARCHAR NOT NULL,
    title VARCHAR NOT NULL
);

CREATE TABLE division (
    id INTEGER PRIMARY KEY,
    d_name VARCHAR NOT NULL
);

CREATE TABLE member (
    id SERIAL PRIMARY KEY,
    m_name VARCHAR NOT NULL,
    profile_id INTEGER REFERENCES images,
    role VARCHAR DEFAULT 'member' NOT NUll,
    bio VARCHAR DEFAULT 'Hello' NOT NULL,
    joined DATE NOT NULL,
    reported INTEGER NOT NULL DEFAULT 0,
    class VARCHAR NOT NULL,
    division_id INTEGER REFERENCES division
);

CREATE TABLE president (
    id INTEGER PRIMARY KEY REFERENCES member,
    start DATE NOT NULL,
    p_end DATE
);

CREATE TABLE program (
    id INTEGER PRIMARY KEY,
    p_name VARCHAR NOT NULL,
    p_desc VARCHAR NOT NULL,
    date_made DATE NOT NULL,
    date_ended DATE,
    is_public BOOLEAN NOT NULL,
    profile_id INTEGER REFERENCES images,
    president_id INTEGER NOT NULL REFERENCES president,
    status INTEGER NOT NULL
);

CREATE TABLE program_image (
    program_id INTEGER REFERENCES program,
    image_id INTEGER REFERENCES images,
    PRIMARY KEY(program_id, image_id)
);

CREATE TABLE event (
    id INTEGER PRIMARY KEY,
    e_name VARCHAR NOT NULL,
    e_desc VARCHAR NOT NULL,
    e_date DATE NOT NULL,
    is_public BOOLEAN NOT NULL,
    profile_id INTEGER NOT NULL REFERENCES images,
    program_id INTEGER NOT NULL
);

CREATE TABLE improvement (
    id INTEGER PRIMARY KEY,
    body VARCHAR NOT NULL,
    likes INTEGER NOT NULL DEFAULT 0,
    dislikes INTEGER NOT NULL DEFAULT 0,
    event_id INTEGER REFERENCES event,
    program_id INTEGER REFERENCES program
);

CREATE TABLE schedule (
    id INTEGER PRIMARY KEY,
    agenda VARCHAR NOT NULL,
    start TIMESTAMP NOT NULL,
    s_length INTERVAL NOT NULL,
    location VARCHAR DEFAULT 'EPISJH Lobby' NOT NULL,
    details VARCHAR,
    event_id INTEGER NOT NULL REFERENCES event
);

CREATE TABLE worked_event (
    event_id INTEGER REFERENCES event,
    division_id INTEGER REFERENCES division,
    PRIMARY KEY(event_id, division_id)
);

CREATE TABLE worked_program (
    program_id INTEGER REFERENCES program,
    division_ID INTEGER REFERENCES division,
    PRIMARY KEY(program_id, division_id)
);

CREATE TABLE reviews (
    id INTEGER PRIMARY KEY REFERENCES member,
    title VARCHAR NOT NULL,
    body VARCHAR NOT NULL,
    event_id INTEGER REFERENCES event,
    program_id INTEGER REFERENCES program
);

CREATE TABLE event_image (
    event_id INTEGER REFERENCES event,
    image_id INTEGER REFERENCES images,
    PRIMARY KEY(event_id, image_id)
);

CREATE TABLE forums (
    id INTEGER PRIMARY KEY,
    title VARCHAR NOT NULL,
    member_id INTEGER NOT NULL REFERENCES member
);

CREATE TABLE forum (
    id INTEGER PRIMARY KEY,
    f_desc VARCHAR NOT NULL,
    forum_id INTEGER NOT NULL REFERENCES forums,
    member_id INTEGER NOT NULL REFERENCES member
);

CREATE TABLE visit (
    id INTEGER PRIMARY KEY,
    time TIMESTAMP NOT NULL,
    forums_id INTEGER NOT NULL REFERENCES forums
);

CREATE TABLE blog (
    id INTEGER PRIMARY KEY,
    member_id INTEGER REFERENCES member
);

CREATE TABLE blog_post (
    id INTEGER PRIMARY KEY,
    blog_id INTEGER REFERENCES blog
);
