DROP TABLE IF EXISTS `sys_res`;

CREATE TABLE `sys_res`
(
    `id`          TEXT NOT NULL PRIMARY KEY,
    `parent_id`   TEXT DEFAULT NULL,
    `name`        TEXT DEFAULT NULL,
    `permission`  TEXT NOT NULL,
    `path`        TEXT DEFAULT NULL,
    `del`         INTEGER NOT NULL DEFAULT '1',
    `create_date` DATETIME NOT NULL
);

INSERT INTO `sys_res`
VALUES ('1', NULL, '/', '/', '/', 0, '2020-02-09 00:00:00'),
       ('2', NULL, 'dashboard', 'dashboard', 'dashboard', 0, '2020-02-09 00:00:00'),
       ('3', NULL, '首页', '/', '', 0, '2020-08-13 11:43:26'),
       ('4', NULL, 'form', 'form', 'form', 0, '2020-02-09 00:00:00'),
       ('5', NULL, 'table', 'table', 'table', 0, '2020-02-09 00:00:00'),
       ('6', NULL, 'profile', 'profile', 'profile', 0, '2020-02-09 00:00:00'),
       ('7', NULL, 'result', 'result', 'result', 0, '2020-02-09 00:00:00'),
       ('8', NULL, 'exception', 'exception', 'exception', 0, '2020-02-09 00:00:00'),
       ('9', NULL, 'user', 'user', 'user', 0, '2020-02-09 00:00:00'),
       ('10', NULL, 'setting', 'setting', 'setting', 0, '2020-02-09 00:00:00');

DROP TABLE IF EXISTS `sys_role`;

CREATE TABLE `sys_role`
(
    `id`          TEXT NOT NULL PRIMARY KEY,
    `name`        TEXT DEFAULT NULL,
    `del`         INTEGER NOT NULL DEFAULT '1',
    `create_date` DATETIME NOT NULL,
    `parent_id`   TEXT DEFAULT NULL
);

INSERT INTO `sys_role`
VALUES ('1', 'super', 0, '2020-07-28 08:34:40', NULL);


DROP TABLE IF EXISTS `sys_role_res`;

CREATE TABLE `sys_role_res`
(
    `id`          TEXT PRIMARY KEY,
    `role_id`     TEXT NOT NULL,
    `res_id`      TEXT NOT NULL,
    `create_date` TEXT NOT NULL
);

INSERT INTO `sys_role_res`
VALUES ('1', '1', '1', '2020-07-28 08:34:40'),
       ('2', '1', '2', '2020-07-28 08:34:40'),
       ('3', '1', '3', '2020-07-28 08:34:40'),
       ('4', '1', '4', '2020-07-28 08:34:40'),
       ('5', '1', '5', '2020-07-28 08:34:40'),
       ('6', '1', '6', '2020-07-28 08:34:40'),
       ('7', '1', '7', '2020-07-28 08:34:40'),
       ('8', '1', '8', '2020-07-28 08:34:40'),
       ('9', '1', '9', '2020-07-28 08:34:40'),
       ('10', '1', '10', '2020-07-28 08:34:40');

DROP TABLE IF EXISTS `sys_user`;

CREATE TABLE `sys_user`
(
    `id`          TEXT PRIMARY KEY,
    `account`     TEXT NOT NULL,
    `password`    TEXT NOT NULL,
    `name`        TEXT DEFAULT NULL,
    `login_check` TEXT DEFAULT 'PasswordQRCodeCheck',
    `del`         INTEGER NOT NULL DEFAULT '1',
    `create_date` TEXT NOT NULL,
    `state`       INTEGER NOT NULL DEFAULT '1'
);

INSERT INTO `sys_user`
VALUES ('1', '00000000000', 'e10adc3949ba59abbe56e057f20f883e', '超级管理员', 'PasswordCheck', 0,
        '2020-07-28 08:34:40', 1);

DROP TABLE IF EXISTS `sys_user_role`;

CREATE TABLE `sys_user_role`
(
    `id`          TEXT PRIMARY KEY,
    `user_id`     TEXT NOT NULL,
    `role_id`     TEXT NOT NULL,
    `create_date` TEXT NOT NULL
);

INSERT INTO `sys_user_role`
VALUES ('1', '1', '1', '2020-07-28 08:34:40');

DROP TABLE IF EXISTS `sys_dict`;

CREATE TABLE `sys_dict`
(
    `id`          TEXT PRIMARY KEY,
    `name`        TEXT NOT NULL,
    `code`        TEXT NOT NULL,
    `state`       INTEGER NOT NULL,
    `create_date` TEXT NOT NULL
);

CREATE TABLE sys_trash (
                           id          TEXT NOT NULL,
                           table_name  TEXT NOT NULL,
                           data        TEXT,
                           create_date DATETIME
);

