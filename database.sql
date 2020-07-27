
--
-- Table structure for table `biz_activity`
--

DROP TABLE IF EXISTS `biz_activity`;
 SET character_set_client = utf8mb4 ;
CREATE TABLE `biz_activity` (
  `id` varchar(50) NOT NULL DEFAULT '' COMMENT '唯一活动码',
  `name` varchar(255) NOT NULL,
  `pc_link` varchar(255) DEFAULT NULL,
  `h5_link` varchar(255) DEFAULT NULL,
  `sort` varchar(255) NOT NULL COMMENT '排序',
  `status` int(11) NOT NULL COMMENT '状态（0：已下线，1：已上线）',
  `version` int(11) NOT NULL,
  `remark` varchar(255) DEFAULT NULL,
  `create_time` datetime NOT NULL,
  `delete_flag` int(1) NOT NULL,
  `pc_banner_img` varchar(255) DEFAULT NULL,
  `h5_banner_img` varchar(255) DEFAULT NULL,
  PRIMARY KEY (`id`) USING BTREE
) ENGINE=InnoDB DEFAULT CHARSET=utf8 ROW_FORMAT=COMPACT COMMENT='运营管理-活动管理';

--
-- Dumping data for table `biz_activity`
--

LOCK TABLES `biz_activity` WRITE;
INSERT INTO `biz_activity` VALUES ('12312','null','null','null','null',1,1,'null','2020-02-09 00:00:00',1,'null','null'),('178','test_insret','','','1',1,0,'','2020-06-17 20:08:13',1,NULL,NULL),('221','test','','','0',0,0,'','2020-06-17 20:10:23',1,NULL,NULL),('222','test','','','0',0,0,'','2020-06-17 20:10:23',1,NULL,NULL),('223','test','','','0',0,0,'','2020-06-17 20:10:23',1,NULL,NULL);
UNLOCK TABLES;

--
-- Table structure for table `biz_admin_user`
--

DROP TABLE IF EXISTS `biz_admin_user`;
 SET character_set_client = utf8mb4 ;
CREATE TABLE `biz_admin_user` (
  `id` varchar(45) CHARACTER SET latin1 NOT NULL,
  `account` varchar(45) CHARACTER SET latin1 NOT NULL,
  `password` varchar(255) CHARACTER SET latin1 NOT NULL,
  `name` varchar(255) CHARACTER SET latin1 DEFAULT NULL,
  `del` int(1) NOT NULL DEFAULT '1',
  `create_time` datetime NOT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8;

--
-- Dumping data for table `biz_admin_user`
--

LOCK TABLES `biz_admin_user` WRITE;
UNLOCK TABLES;

--
-- Table structure for table `biz_admin_user_role`
--

DROP TABLE IF EXISTS `biz_admin_user_role`;
 SET character_set_client = utf8mb4 ;
CREATE TABLE `biz_admin_user_role` (
  `id` varchar(45) CHARACTER SET latin1 NOT NULL,
  `user_id` varchar(45) CHARACTER SET latin1 NOT NULL,
  `role_id` varchar(45) CHARACTER SET latin1 NOT NULL,
  `create_time` datetime NOT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8;

--
-- Dumping data for table `biz_admin_user_role`
--

LOCK TABLES `biz_admin_user_role` WRITE;
UNLOCK TABLES;

--
-- Table structure for table `biz_res`
--

DROP TABLE IF EXISTS `biz_res`;
 SET character_set_client = utf8mb4 ;
CREATE TABLE `biz_res` (
  `id` varchar(45) CHARACTER SET latin1 NOT NULL,
  `parent_id` varchar(45) CHARACTER SET latin1 DEFAULT NULL,
  `name` varchar(255) CHARACTER SET latin1 DEFAULT NULL,
  `permission` varchar(45) CHARACTER SET latin1 NOT NULL,
  `path` varchar(45) CHARACTER SET latin1 DEFAULT NULL,
  `del` int(1) NOT NULL DEFAULT '1',
  `create_time` datetime NOT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8 COLLATE=utf8_bin;

--
-- Dumping data for table `biz_res`
--

LOCK TABLES `biz_res` WRITE;
INSERT INTO `biz_res` VALUES ('1',NULL,'qx','res_page','/res_page',1,'2020-02-09 00:00:00');
UNLOCK TABLES;

--
-- Table structure for table `biz_role`
--

DROP TABLE IF EXISTS `biz_role`;
 SET character_set_client = utf8mb4 ;
CREATE TABLE `biz_role` (
  `id` varchar(45) CHARACTER SET latin1 NOT NULL,
  `name` varchar(255) CHARACTER SET latin1 DEFAULT NULL,
  `del` int(1) NOT NULL DEFAULT '1',
  `create_time` datetime NOT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8;

--
-- Dumping data for table `biz_role`
--

LOCK TABLES `biz_role` WRITE;
UNLOCK TABLES;

--
-- Table structure for table `biz_role_res`
--

DROP TABLE IF EXISTS `biz_role_res`;
 SET character_set_client = utf8mb4 ;
CREATE TABLE `biz_role_res` (
  `id` varchar(45) CHARACTER SET latin1 NOT NULL,
  `role_id` varchar(45) CHARACTER SET latin1 NOT NULL,
  `res_id` varchar(45) CHARACTER SET latin1 NOT NULL,
  `create_time` datetime NOT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8;

--
-- Dumping data for table `biz_role_res`
--

LOCK TABLES `biz_role_res` WRITE;
UNLOCK TABLES;
