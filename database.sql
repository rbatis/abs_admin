-- MySQL dump 10.13  Distrib 8.0.23, for Win64 (x86_64)
--
-- Host: 127.0.0.1    Database: test
-- ------------------------------------------------------
-- Server version	5.7.33

/*!40101 SET @OLD_CHARACTER_SET_CLIENT=@@CHARACTER_SET_CLIENT */;
/*!40101 SET @OLD_CHARACTER_SET_RESULTS=@@CHARACTER_SET_RESULTS */;
/*!40101 SET @OLD_COLLATION_CONNECTION=@@COLLATION_CONNECTION */;
/*!50503 SET NAMES utf8 */;
/*!40103 SET @OLD_TIME_ZONE=@@TIME_ZONE */;
/*!40103 SET TIME_ZONE='+00:00' */;
/*!40014 SET @OLD_UNIQUE_CHECKS=@@UNIQUE_CHECKS, UNIQUE_CHECKS=0 */;
/*!40014 SET @OLD_FOREIGN_KEY_CHECKS=@@FOREIGN_KEY_CHECKS, FOREIGN_KEY_CHECKS=0 */;
/*!40101 SET @OLD_SQL_MODE=@@SQL_MODE, SQL_MODE='NO_AUTO_VALUE_ON_ZERO' */;
/*!40111 SET @OLD_SQL_NOTES=@@SQL_NOTES, SQL_NOTES=0 */;

--
-- Table structure for table `biz_activity`
--

DROP TABLE IF EXISTS `biz_activity`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `biz_activity` (
                                `id` varchar(50) NOT NULL DEFAULT '' COMMENT '唯一活动码',
                                `name` varchar(255) NOT NULL,
                                `pc_link` varchar(255) DEFAULT NULL,
                                `h5_link` varchar(255) DEFAULT NULL,
                                `sort` varchar(255) NOT NULL COMMENT '排序',
                                `status` int(11) NOT NULL COMMENT '状态（0：已下线，1：已上线）',
                                `version` int(11) NOT NULL,
                                `remark` varchar(255) DEFAULT NULL,
                                `create_date` datetime NOT NULL,
                                `delete_flag` int(1) NOT NULL,
                                `pc_banner_img` varchar(255) DEFAULT NULL,
                                `h5_banner_img` varchar(255) DEFAULT NULL,
                                PRIMARY KEY (`id`) USING BTREE
) ENGINE=InnoDB DEFAULT CHARSET=utf8 ROW_FORMAT=COMPACT COMMENT='运营管理-活动管理';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `biz_activity`
--

LOCK TABLES `biz_activity` WRITE;
/*!40000 ALTER TABLE `biz_activity` DISABLE KEYS */;
INSERT INTO `biz_activity` VALUES ('1','test','','','0',0,0,'','2020-06-17 20:10:23',1,NULL,NULL),('12312','123',NULL,NULL,'1',1,1,NULL,'2020-09-06 16:09:02',0,NULL,NULL),('178','test_insret','','','1',1,0,'','2020-06-17 20:08:13',0,NULL,NULL),('221','test','','','0',0,0,'','2020-06-17 20:10:23',0,NULL,NULL),('222','test','','','0',0,0,'','2020-06-17 20:10:23',0,NULL,NULL);
/*!40000 ALTER TABLE `biz_activity` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `sys_res`
--

DROP TABLE IF EXISTS `sys_res`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `sys_res` (
                           `id` varchar(45) CHARACTER SET latin1 NOT NULL,
                           `parent_id` varchar(45) CHARACTER SET latin1 DEFAULT NULL,
                           `name` varchar(255) COLLATE utf8_bin DEFAULT NULL,
                           `permission` varchar(45) CHARACTER SET latin1 NOT NULL,
                           `path` varchar(45) CHARACTER SET latin1 DEFAULT NULL,
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
INSERT INTO `sys_res` VALUES ('1',NULL,'qx','/','/',0,'2020-02-09 00:00:00'),('2',NULL,'qx','dashboard','dashboard',0,'2020-02-09 00:00:00'),('3',NULL,'qx','form','form',0,'2020-02-09 00:00:00'),('4',NULL,'qx','table','table',0,'2020-02-09 00:00:00'),('5',NULL,'qx','profile','profile',0,'2020-02-09 00:00:00'),('6',NULL,'qx','result','result',0,'2020-02-09 00:00:00'),('7',NULL,'qx','exception','exception',0,'2020-02-09 00:00:00'),('8',NULL,'qx','user','user',0,'2020-02-09 00:00:00'),('9',NULL,'qx','setting','setting',0,'2020-02-09 00:00:00'),('206267260095041511',NULL,'超级管理员权限','/','',0,'2020-08-13 11:43:26');
/*!40000 ALTER TABLE `sys_res` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `sys_role`
--

DROP TABLE IF EXISTS `sys_role`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `sys_role` (
                            `id` varchar(45) CHARACTER SET latin1 NOT NULL,
                            `name` varchar(255) CHARACTER SET latin1 DEFAULT NULL,
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
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `sys_role_res` (
                                `id` varchar(45) CHARACTER SET latin1 NOT NULL,
                                `role_id` varchar(45) CHARACTER SET latin1 NOT NULL,
                                `res_id` varchar(45) CHARACTER SET latin1 NOT NULL,
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
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `sys_user` (
                            `id` varchar(45) NOT NULL,
                            `account` varchar(45) NOT NULL,
                            `password` varchar(255) NOT NULL,
                            `name` varchar(255) DEFAULT NULL,
                            `login_check` varchar(255) DEFAULT 'PasswordQRCodeCheck',
                            `del` int(1) NOT NULL DEFAULT '1',
                            `create_date` datetime NOT NULL,
                            PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `sys_user`
--

LOCK TABLES `sys_user` WRITE;
/*!40000 ALTER TABLE `sys_user` DISABLE KEYS */;
INSERT INTO `sys_user` VALUES ('2d4886bd-ad2a-4644-86b9-460afad05cbf','00000000000','e10adc3949ba59abbe56e057f20f883e','xxxx','PasswordCheck',0,'2020-07-28 08:34:40'),('7c0591a3-41f9-423b-943e-6944d7760192','18969542174','e10adc3949ba59abbe56e057f20f883e','xxxx','PasswordCheck',0,'2020-08-07 17:01:02'),('91095081-1966-491d-a95d-bd1bc36d204f','1896954211','e10adc3949ba59abbe56e057f20f883e','xxxx','PasswordCheck',0,'2020-08-31 20:05:26'),('f63ef5b6-171e-47ab-8aad-832ff4323056','18969542173','e10adc3949ba59abbe56e057f20f883e','xxxx','PasswordCheck',0,'2020-07-31 05:47:28'),('f84ad1fe-1d96-4776-bb2b-45fc990a9975','18969542171','e10adc3949ba59abbe56e057f20f883e','xxxx','PasswordCheck',0,'2020-08-07 16:54:45');
/*!40000 ALTER TABLE `sys_user` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `sys_user_role`
--

DROP TABLE IF EXISTS `sys_user_role`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `sys_user_role` (
                                 `id` varchar(45) CHARACTER SET latin1 NOT NULL,
                                 `user_id` varchar(45) CHARACTER SET latin1 NOT NULL,
                                 `role_id` varchar(45) CHARACTER SET latin1 NOT NULL,
                                 `create_date` datetime NOT NULL,
                                 PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `sys_user_role`
--

LOCK TABLES `sys_user_role` WRITE;
/*!40000 ALTER TABLE `sys_user_role` DISABLE KEYS */;
INSERT INTO `sys_user_role` VALUES ('1','2d4886bd-ad2a-4644-86b9-460afad05cbf','1','2020-07-28 08:34:40');
/*!40000 ALTER TABLE `sys_user_role` ENABLE KEYS */;
UNLOCK TABLES;
/*!40103 SET TIME_ZONE=@OLD_TIME_ZONE */;

/*!40101 SET SQL_MODE=@OLD_SQL_MODE */;
/*!40014 SET FOREIGN_KEY_CHECKS=@OLD_FOREIGN_KEY_CHECKS */;
/*!40014 SET UNIQUE_CHECKS=@OLD_UNIQUE_CHECKS */;
/*!40101 SET CHARACTER_SET_CLIENT=@OLD_CHARACTER_SET_CLIENT */;
/*!40101 SET CHARACTER_SET_RESULTS=@OLD_CHARACTER_SET_RESULTS */;
/*!40101 SET COLLATION_CONNECTION=@OLD_COLLATION_CONNECTION */;
/*!40111 SET SQL_NOTES=@OLD_SQL_NOTES */;

-- Dump completed on 2021-02-17 19:16:54
