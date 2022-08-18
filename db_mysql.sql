-- MySQL dump 10.13  Distrib 8.0.16, for macos10.14 (x86_64)
--
-- Host: 127.0.0.1    Database: test
-- ------------------------------------------------------
-- Server version	5.7.33

--
-- Table structure for table `sys_dict`
--

DROP TABLE IF EXISTS `sys_dict`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
 SET character_set_client = utf8mb4 ;
CREATE TABLE `sys_dict` (
  `id` varchar(45) COLLATE utf8mb4_unicode_ci NOT NULL COMMENT '键',
  `parent_id` varchar(45) COLLATE utf8mb4_unicode_ci DEFAULT NULL COMMENT '父级键',
  `name` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL,
  `code` varchar(45) COLLATE utf8mb4_unicode_ci NOT NULL,
  `state` int(1) NOT NULL DEFAULT '1' COMMENT '该字段是否有效，默认1有效0无效',
  `create_date` datetime DEFAULT NULL COMMENT '创建时间',
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci COMMENT='系统字典表';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `sys_dict`
--

LOCK TABLES `sys_dict` WRITE;
/*!40000 ALTER TABLE `sys_dict` DISABLE KEYS */;
INSERT INTO `sys_dict` VALUES ('270804252920778752',NULL,'使用状态','use_state',1,'2021-08-19 14:39:25'),('270835898336284672','270804252920778752','禁用','use_state_0',1,'2021-08-19 16:45:10'),('270838503200395264','270804252920778752','启用','use_state_1',1,'2021-08-19 16:55:31');
/*!40000 ALTER TABLE `sys_dict` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `sys_res`
--

DROP TABLE IF EXISTS `sys_res`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
 SET character_set_client = utf8mb4 ;
CREATE TABLE `sys_res` (
  `id` varchar(45) CHARACTER SET utf8 NOT NULL,
  `parent_id` varchar(45) CHARACTER SET utf8 DEFAULT NULL,
  `name` varchar(255) COLLATE utf8_bin DEFAULT NULL,
  `permission` varchar(45) CHARACTER SET utf8 NOT NULL,
  `path` varchar(45) CHARACTER SET utf8 DEFAULT NULL,
  `del` int(1) NOT NULL DEFAULT '1',
  `create_date` datetime NOT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8 COLLATE=utf8_bin;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `sys_res`
--

LOCK TABLES `sys_res` WRITE;
/*!40000 ALTER TABLE `sys_res` DISABLE KEYS */;
INSERT INTO `sys_res` VALUES ('1',NULL,'/','/','/',0,'2020-02-09 00:00:00'),('2',NULL,'dashboard','dashboard','dashboard',0,'2020-02-09 00:00:00'),('206267260095041511',NULL,'首页','/','',0,'2020-08-13 11:43:26'),('3',NULL,'form','form','form',0,'2020-02-09 00:00:00'),('4',NULL,'table','table','table',0,'2020-02-09 00:00:00'),('5',NULL,'profile','profile','profile',0,'2020-02-09 00:00:00'),('6',NULL,'result','result','result',0,'2020-02-09 00:00:00'),('7',NULL,'exception','exception','exception',0,'2020-02-09 00:00:00'),('8',NULL,'user','user','user',0,'2020-02-09 00:00:00'),('9',NULL,'setting','setting','setting',0,'2020-02-09 00:00:00');
/*!40000 ALTER TABLE `sys_res` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `sys_role`
--

DROP TABLE IF EXISTS `sys_role`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
 SET character_set_client = utf8mb4 ;
CREATE TABLE `sys_role` (
  `id` varchar(45) CHARACTER SET utf8 NOT NULL,
  `name` varchar(255) CHARACTER SET utf8 DEFAULT NULL,
  `del` int(1) NOT NULL DEFAULT '1',
  `create_date` datetime NOT NULL,
  `parent_id` varchar(255) DEFAULT NULL COMMENT '父id',
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `sys_role`
--

LOCK TABLES `sys_role` WRITE;
/*!40000 ALTER TABLE `sys_role` DISABLE KEYS */;
INSERT INTO `sys_role` VALUES ('1','super',0,'2020-07-28 08:34:40',NULL);
/*!40000 ALTER TABLE `sys_role` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `sys_role_res`
--

DROP TABLE IF EXISTS `sys_role_res`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
 SET character_set_client = utf8mb4 ;
CREATE TABLE `sys_role_res` (
  `id` varchar(45) CHARACTER SET utf8 NOT NULL,
  `role_id` varchar(45) CHARACTER SET utf8 NOT NULL,
  `res_id` varchar(45) CHARACTER SET utf8 NOT NULL,
  `create_date` datetime NOT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `sys_role_res`
--

LOCK TABLES `sys_role_res` WRITE;
/*!40000 ALTER TABLE `sys_role_res` DISABLE KEYS */;
INSERT INTO `sys_role_res` VALUES ('1','1','1','2020-07-28 08:34:40'),('10','1','9','2020-07-28 08:34:40'),('2','1','206267260095041511','2020-07-28 08:34:40'),('3','1','2','2020-07-28 08:34:40'),('4','1','3','2020-07-28 08:34:40'),('5','1','4','2020-07-28 08:34:40'),('6','1','5','2020-07-28 08:34:40'),('7','1','6','2020-07-28 08:34:40'),('8','1','7','2020-07-28 08:34:40'),('9','1','8','2020-07-28 08:34:40');
/*!40000 ALTER TABLE `sys_role_res` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `sys_user`
--

DROP TABLE IF EXISTS `sys_user`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
 SET character_set_client = utf8mb4 ;
CREATE TABLE `sys_user` (
  `id` varchar(45) NOT NULL,
  `account` varchar(45) NOT NULL,
  `password` varchar(255) NOT NULL,
  `name` varchar(255) DEFAULT NULL,
  `login_check` varchar(255) DEFAULT 'PasswordQRCodeCheck',
  `del` int(1) NOT NULL DEFAULT '1',
  `create_date` datetime NOT NULL,
  `state` int(1) NOT NULL DEFAULT '1',
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `sys_user`
--

LOCK TABLES `sys_user` WRITE;
/*!40000 ALTER TABLE `sys_user` DISABLE KEYS */;
INSERT INTO `sys_user` VALUES ('205667537625681919','00000000000','e10adc3949ba59abbe56e057f20f883e','超级管理员','PasswordCheck',0,'2020-07-28 08:34:40',1),('270466997106642944','15625285826','e10adc3949ba59abbe56e057f20f883e','johnson',NULL,0,'2021-08-18 16:19:17',1);
/*!40000 ALTER TABLE `sys_user` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `sys_user_role`
--

DROP TABLE IF EXISTS `sys_user_role`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
 SET character_set_client = utf8mb4 ;
CREATE TABLE `sys_user_role` (
  `id` varchar(45) CHARACTER SET utf8 NOT NULL,
  `user_id` varchar(45) CHARACTER SET utf8 NOT NULL,
  `role_id` varchar(45) CHARACTER SET utf8 NOT NULL,
  `create_date` datetime NOT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `sys_user_role`
--

LOCK TABLES `sys_user_role` WRITE;
/*!40000 ALTER TABLE `sys_user_role` DISABLE KEYS */;
INSERT INTO `sys_user_role` VALUES ('1','205667537625681919','1','2020-07-28 08:34:40'),('270466997110837248','270466997106642944','1','2021-08-18 16:19:17');
/*!40000 ALTER TABLE `sys_user_role` ENABLE KEYS */;
UNLOCK TABLES;



DROP TABLE IF EXISTS `sys_dict`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
SET character_set_client = utf8mb4 ;
CREATE TABLE `sys_dict` (
                                 `id` varchar(45)  NOT NULL,
                                 `name` varchar(255)  NOT NULL,
                                 `code` varchar(512)  NOT NULL,
                                 `state` int(11)  NOT NULL,
                                 `create_date` datetime NOT NULL,
                                 PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8;


create table sys_trash
(
    id          varchar(255) not null,
    table_name       varchar(255) not null,
    data        text         null,
    create_date datetime     null
);

