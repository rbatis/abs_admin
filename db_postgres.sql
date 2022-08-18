

-- ----------------------------
-- Table structure for sys_dict
-- ----------------------------
DROP TABLE IF EXISTS "public"."sys_dict";
CREATE TABLE "public"."sys_dict" (
  "id" varchar(256) COLLATE "pg_catalog"."default" NOT NULL,
  "name" varchar(512) COLLATE "pg_catalog"."default" NOT NULL,
  "code" varchar(512) COLLATE "pg_catalog"."default" NOT NULL,
  "state" int4 NOT NULL,
  "create_date" timestamp(6) NOT NULL
)
;
ALTER TABLE "public"."sys_dict" OWNER TO "postgres";

-- ----------------------------
-- Records of sys_dict
-- ----------------------------
BEGIN;
COMMIT;

-- ----------------------------
-- Table structure for sys_res
-- ----------------------------
DROP TABLE IF EXISTS "public"."sys_res";
CREATE TABLE "public"."sys_res" (
  "id" varchar(256) COLLATE "pg_catalog"."default" NOT NULL,
  "parent_id" varchar(256) COLLATE "pg_catalog"."default" DEFAULT NULL::character varying,
  "name" varchar(256) COLLATE "pg_catalog"."default" DEFAULT NULL::character varying,
  "permission" varchar(256) COLLATE "pg_catalog"."default" NOT NULL,
  "path" varchar(256) COLLATE "pg_catalog"."default" DEFAULT NULL::character varying,
  "del" int4 DEFAULT 1,
  "create_date" timestamp(6) NOT NULL
)
;
ALTER TABLE "public"."sys_res" OWNER TO "postgres";
COMMENT ON COLUMN "public"."sys_res"."create_date" IS '创建时间';

-- ----------------------------
-- Records of sys_res
-- ----------------------------
BEGIN;
INSERT INTO "public"."sys_res" VALUES ('1', NULL, '/', '/', '/', 0, '2020-02-09 00:00:00');
INSERT INTO "public"."sys_res" VALUES ('2', NULL, 'dashboard', 'dashboard', 'dashboard', 0, '2020-02-09 00:00:00');
INSERT INTO "public"."sys_res" VALUES ('3', NULL, 'form', 'form', 'form', 0, '2020-02-09 00:00:00');
INSERT INTO "public"."sys_res" VALUES ('4', NULL, 'table', 'table', 'table', 0, '2020-02-09 00:00:00');
INSERT INTO "public"."sys_res" VALUES ('5', NULL, 'profile', 'profile', 'profile', 0, '2020-02-09 00:00:00');
INSERT INTO "public"."sys_res" VALUES ('6', NULL, 'result', 'result', 'result', 0, '2020-02-09 00:00:00');
INSERT INTO "public"."sys_res" VALUES ('7', NULL, 'exception', 'exception', 'exception', 0, '2020-02-09 00:00:00');
INSERT INTO "public"."sys_res" VALUES ('8', NULL, 'user', 'user', 'user', 0, '2020-02-09 00:00:00');
INSERT INTO "public"."sys_res" VALUES ('9', NULL, 'setting', 'setting', 'setting', 0, '2020-02-09 00:00:00');
INSERT INTO "public"."sys_res" VALUES ('206267260095041511', NULL, '超级管理员权限', '/', '', 0, '2020-08-13 11:43:26');
COMMIT;

-- ----------------------------
-- Table structure for sys_role
-- ----------------------------
DROP TABLE IF EXISTS "public"."sys_role";
CREATE TABLE "public"."sys_role" (
  "id" varchar(256) COLLATE "pg_catalog"."default" NOT NULL,
  "name" varchar(256) COLLATE "pg_catalog"."default" DEFAULT NULL::character varying,
  "del" int4 NOT NULL DEFAULT 1,
  "create_date" timestamp(6) NOT NULL,
  "parent_id" varchar(255) COLLATE "pg_catalog"."default" DEFAULT NULL::character varying
)
;
ALTER TABLE "public"."sys_role" OWNER TO "postgres";
COMMENT ON COLUMN "public"."sys_role"."parent_id" IS '父id';

-- ----------------------------
-- Records of sys_role
-- ----------------------------
BEGIN;
INSERT INTO "public"."sys_role" VALUES ('1', 'super', 0, '2020-07-28 08:34:40', NULL);
COMMIT;

-- ----------------------------
-- Table structure for sys_role_res
-- ----------------------------
DROP TABLE IF EXISTS "public"."sys_role_res";
CREATE TABLE "public"."sys_role_res" (
  "id" varchar(256) COLLATE "pg_catalog"."default" NOT NULL,
  "role_id" varchar(256) COLLATE "pg_catalog"."default" NOT NULL,
  "res_id" varchar(256) COLLATE "pg_catalog"."default" NOT NULL,
  "create_date" timestamp(6) NOT NULL
)
;
ALTER TABLE "public"."sys_role_res" OWNER TO "postgres";

-- ----------------------------
-- Records of sys_role_res
-- ----------------------------
BEGIN;
INSERT INTO "public"."sys_role_res" VALUES ('1', '1', '1', '2020-07-28 08:34:40');
INSERT INTO "public"."sys_role_res" VALUES ('10', '1', '9', '2020-07-28 08:34:40');
INSERT INTO "public"."sys_role_res" VALUES ('2', '1', '206267260095041511', '2020-07-28 08:34:40');
INSERT INTO "public"."sys_role_res" VALUES ('3', '1', '2', '2020-07-28 08:34:40');
INSERT INTO "public"."sys_role_res" VALUES ('4', '1', '3', '2020-07-28 08:34:40');
INSERT INTO "public"."sys_role_res" VALUES ('5', '1', '4', '2020-07-28 08:34:40');
INSERT INTO "public"."sys_role_res" VALUES ('6', '1', '5', '2020-07-28 08:34:40');
INSERT INTO "public"."sys_role_res" VALUES ('7', '1', '6', '2020-07-28 08:34:40');
INSERT INTO "public"."sys_role_res" VALUES ('8', '1', '7', '2020-07-28 08:34:40');
INSERT INTO "public"."sys_role_res" VALUES ('9', '1', '8', '2020-07-28 08:34:40');
COMMIT;

-- ----------------------------
-- Table structure for sys_user
-- ----------------------------
DROP TABLE IF EXISTS "public"."sys_user";
CREATE TABLE "public"."sys_user" (
  "id" varchar(45) COLLATE "pg_catalog"."default" NOT NULL,
  "account" varchar(45) COLLATE "pg_catalog"."default" NOT NULL,
  "password" varchar(255) COLLATE "pg_catalog"."default" NOT NULL,
  "name" varchar(255) COLLATE "pg_catalog"."default" DEFAULT NULL::character varying,
  "login_check" varchar(255) COLLATE "pg_catalog"."default" DEFAULT 'PasswordQRCodeCheck'::character varying,
  "del" int4 NOT NULL DEFAULT 1,
  "create_date" timestamp(6) NOT NULL,
  "state" int4 NOT NULL DEFAULT 1
)
;
ALTER TABLE "public"."sys_user" OWNER TO "postgres";

-- ----------------------------
-- Records of sys_user
-- ----------------------------
BEGIN;
INSERT INTO "public"."sys_user" VALUES ('205667537625681919', '00000000000', 'e10adc3949ba59abbe56e057f20f883e', '超级管理员', 'PasswordCheck', 0, '2020-07-28 08:34:40', 1);
COMMIT;

-- ----------------------------
-- Table structure for sys_user_role
-- ----------------------------
DROP TABLE IF EXISTS "public"."sys_user_role";
CREATE TABLE "public"."sys_user_role" (
  "id" varchar(256) COLLATE "pg_catalog"."default" NOT NULL,
  "user_id" varchar(256) COLLATE "pg_catalog"."default" NOT NULL,
  "role_id" varchar(256) COLLATE "pg_catalog"."default" NOT NULL,
  "create_date" timestamp(6) NOT NULL
)
;
ALTER TABLE "public"."sys_user_role" OWNER TO "postgres";


create table sys_trash
(
    id          varchar(256) not null,
    table_name       varchar(256) not null,
    data        text         null,
    create_date timestamp(6) null
);



-- ----------------------------
-- Records of sys_user_role
-- ----------------------------
BEGIN;
INSERT INTO "public"."sys_user_role" VALUES ('1', '205667537625681919', '1', '2020-07-28 08:34:40');
COMMIT;

-- ----------------------------
-- Primary Key structure for table sys_dict
-- ----------------------------
ALTER TABLE "public"."sys_dict" ADD CONSTRAINT "sys_dict_pkey" PRIMARY KEY ("id");

-- ----------------------------
-- Primary Key structure for table sys_res
-- ----------------------------
ALTER TABLE "public"."sys_res" ADD CONSTRAINT "sys_res_pkey" PRIMARY KEY ("id");

-- ----------------------------
-- Primary Key structure for table sys_role
-- ----------------------------
ALTER TABLE "public"."sys_role" ADD CONSTRAINT "sys_role_pkey" PRIMARY KEY ("id");

-- ----------------------------
-- Primary Key structure for table sys_role_res
-- ----------------------------
ALTER TABLE "public"."sys_role_res" ADD CONSTRAINT "sys_role_res_pkey" PRIMARY KEY ("id");

-- ----------------------------
-- Primary Key structure for table sys_user
-- ----------------------------
ALTER TABLE "public"."sys_user" ADD CONSTRAINT "sys_user_pkey" PRIMARY KEY ("id");

-- ----------------------------
-- Primary Key structure for table sys_user_role
-- ----------------------------
ALTER TABLE "public"."sys_user_role" ADD CONSTRAINT "sys_user_role_pkey" PRIMARY KEY ("id");
