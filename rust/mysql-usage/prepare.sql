DROP TABLE IF EXISTS `people`;
CREATE TABLE `people` (
    `id`         BIGINT UNSIGNED PRIMARY KEY AUTO_INCREMENT
,   `name`       VARCHAR(100) NOT NULL
,   `age`        INTEGER
,   `gender`     INTEGER
,   `hash`       CHAR(64)
,   `available`  BOOLEAN NOT NULL DEFAULT TRUE
,   `data`       BLOB
,   `created_at` DATETIME DEFAULT CURRENT_TIMESTAMP
);
