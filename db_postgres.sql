CREATE TABLE IF NOT EXISTS public.sys_res
(
    id          character varying(256) NOT NULL,
    parent_id   character varying(256) DEFAULT NULL,
    name        character varying(256) DEFAULT NULL,
    permission  character varying(256) NOT NULL,
    path        character varying(256) DEFAULT NULL,
    del         integer                DEFAULT '1',
    create_date timestamp              NOT NULL,
    PRIMARY KEY (id)
    );

INSERT INTO sys_res
VALUES ('1', NULL, 'qx', '/', '/', 0, '2020-02-09 00:00:00'),
       ('2', NULL, 'qx', 'dashboard', 'dashboard', 0, '2020-02-09 00:00:00'),
       ('3', NULL, 'qx', 'form', 'form', 0, '2020-02-09 00:00:00'),
       ('4', NULL, 'qx', 'table', 'table', 0, '2020-02-09 00:00:00'),
       ('5', NULL, 'qx', 'profile', 'profile', 0, '2020-02-09 00:00:00'),
       ('6', NULL, 'qx', 'result', 'result', 0, '2020-02-09 00:00:00'),
       ('7', NULL, 'qx', 'exception', 'exception', 0, '2020-02-09 00:00:00'),
       ('8', NULL, 'qx', 'user', 'user', 0, '2020-02-09 00:00:00'),
       ('9', NULL, 'qx', 'setting', 'setting', 0, '2020-02-09 00:00:00'),
       ('206267260095041511', NULL, '超级管理员权限', '/', '', 0, '2020-08-13 11:43:26');


CREATE TABLE IF NOT EXISTS public.sys_role
(
    id          character varying(256) NOT NULL,
    name        character varying(256)          DEFAULT NULL,
    del         integer                NOT NULL DEFAULT '1',
    create_date timestamp              NOT NULL,
    parent_id   varchar(255)                    DEFAULT NULL COMMENT '父id',
    PRIMARY KEY (id)
    );

INSERT INTO sys_role
VALUES ('1', 'super', 0, '2020-07-28 08:34:40', NULL);

CREATE TABLE IF NOT EXISTS public.sys_role_res
(
    id          character varying(256) NOT NULL,
    role_id     character varying(256) NOT NULL,
    res_id      character varying(256) NOT NULL,
    create_date timestamp              NOT NULL,
    PRIMARY KEY (id)
    );

INSERT INTO sys_role_res
VALUES ('1', '1', '1', '2020-07-28 08:34:40'),
       ('10', '1', '9', '2020-07-28 08:34:40'),
       ('2', '1', '206267260095041511', '2020-07-28 08:34:40'),
       ('3', '1', '2', '2020-07-28 08:34:40'),
       ('4', '1', '3', '2020-07-28 08:34:40'),
       ('5', '1', '4', '2020-07-28 08:34:40'),
       ('6', '1', '5', '2020-07-28 08:34:40'),
       ('7', '1', '6', '2020-07-28 08:34:40'),
       ('8', '1', '7', '2020-07-28 08:34:40'),
       ('9', '1', '8', '2020-07-28 08:34:40');

CREATE TABLE IF NOT EXISTS public.sys_user
(
    id          varchar(45)  NOT NULL,
    account     varchar(45)  NOT NULL,
    password    varchar(255) NOT NULL,
    name        varchar(255)          DEFAULT NULL,
    login_check varchar(255)          DEFAULT 'PasswordQRCodeCheck',
    del         integer      NOT NULL DEFAULT '1',
    create_date timestamp    NOT NULL,
    state       integer      NOT NULL DEFAULT '1',
    PRIMARY KEY (id)
    );

INSERT INTO sys_user
VALUES ('205667537625681919', '00000000000', 'e10adc3949ba59abbe56e057f20f883e', 'xxxx', 'PasswordCheck', 0,
        '2020-07-28 08:34:40', 1);

CREATE TABLE IF NOT EXISTS public.sys_user_role
(
    id          character varying(256) NOT NULL,
    user_id     character varying(256) NOT NULL,
    role_id     character varying(256) NOT NULL,
    create_date timestamp              NOT NULL,
    PRIMARY KEY (id)
    );

INSERT INTO sys_user_role VALUES ('1','2d4886bd-ad2a-4644-86b9-460afad05cbf','1','2020-07-28 08:34:40');