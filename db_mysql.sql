
DROP TABLE IF EXISTS `sys_permission`;
SET
character_set_client = utf8mb4 ;
CREATE TABLE `sys_permission`
(
    `id`          varchar(45) CHARACTER SET utf8 NOT NULL,
    `parent_id`   varchar(45) CHARACTER SET utf8 DEFAULT NULL,
    `name`        varchar(255) COLLATE utf8_bin  DEFAULT NULL,
    `permission`  varchar(45) CHARACTER SET utf8 NOT NULL,
    `path`        varchar(45) CHARACTER SET utf8 DEFAULT NULL,
    `del`         int(1) NOT NULL DEFAULT '1',
    `create_date` datetime                       NOT NULL,
    PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8 COLLATE=utf8_bin;

LOCK
TABLES `sys_permission` WRITE;
INSERT INTO `sys_permission`
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
UNLOCK
TABLES;


DROP TABLE IF EXISTS `sys_role`;
SET
character_set_client = utf8mb4 ;
CREATE TABLE `sys_role`
(
    `id`          varchar(45) CHARACTER SET utf8 NOT NULL,
    `name`        varchar(255) CHARACTER SET utf8 DEFAULT NULL,
    `del`         int(1) NOT NULL DEFAULT '1',
    `create_date` datetime                       NOT NULL,
    `parent_id`   varchar(255)                    DEFAULT NULL COMMENT '父id',
    PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8;

LOCK
TABLES `sys_role` WRITE;
INSERT INTO `sys_role`
VALUES ('1', 'super', 0, '2020-07-28 08:34:40', NULL);
UNLOCK
TABLES;


DROP TABLE IF EXISTS `sys_role_permission`;
SET
character_set_client = utf8mb4 ;
CREATE TABLE `sys_role_permission`
(
    `id`          varchar(45) CHARACTER SET utf8 NOT NULL,
    `role_id`     varchar(45) CHARACTER SET utf8 NOT NULL,
    `permission_id`      varchar(45) CHARACTER SET utf8 NOT NULL,
    `create_date` datetime                       NOT NULL,
    PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8;

LOCK
TABLES `sys_role_permission` WRITE;
INSERT INTO `sys_role_permission`
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
UNLOCK
TABLES;

DROP TABLE IF EXISTS `sys_user`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
SET
character_set_client = utf8mb4 ;
CREATE TABLE `sys_user`
(
    `id`          varchar(45)  NOT NULL,
    `account`     varchar(45)  NOT NULL,
    `password`    varchar(255) NOT NULL,
    `name`        varchar(255) DEFAULT NULL,
    `login_check` varchar(255) DEFAULT 'PasswordQRCodeCheck',
    `del`         int(1) NOT NULL DEFAULT '1',
    `create_date` datetime     NOT NULL,
    `state`       int(1) NOT NULL DEFAULT '1',
    PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8;

LOCK
TABLES `sys_user` WRITE;
INSERT INTO `sys_user`
VALUES ('1', '00000000000', 'e10adc3949ba59abbe56e057f20f883e', '超级管理员', 'PasswordCheck', 0,
        '2020-07-28 08:34:40', 1);
UNLOCK
TABLES;

DROP TABLE IF EXISTS `sys_user_role`;
SET
character_set_client = utf8mb4 ;
CREATE TABLE `sys_user_role`
(
    `id`          varchar(45) CHARACTER SET utf8 NOT NULL,
    `user_id`     varchar(45) CHARACTER SET utf8 NOT NULL,
    `role_id`     varchar(45) CHARACTER SET utf8 NOT NULL,
    `create_date` datetime                       NOT NULL,
    PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8;

LOCK
TABLES `sys_user_role` WRITE;
INSERT INTO `sys_user_role`
VALUES ('1', '1', '1', '2020-07-28 08:34:40');
UNLOCK
TABLES;

DROP TABLE IF EXISTS `sys_dict`;
SET
character_set_client = utf8mb4 ;
CREATE TABLE `sys_dict`
(
    `id`          varchar(45)  NOT NULL,
    `name`        varchar(255) NOT NULL,
    `code`        varchar(512) NOT NULL,
    `state`       int(11) NOT NULL,
    `create_date` datetime     NOT NULL,
    PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8;

create table sys_trash
(
    id          varchar(255) not null,
    table_name  varchar(255) not null,
    data        text null,
    create_date datetime null
);