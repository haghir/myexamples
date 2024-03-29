DROP TABLE IF EXISTS `people`;
CREATE TABLE `people` (
    `id`         BIGINT UNSIGNED PRIMARY KEY AUTO_INCREMENT
,   `name`       VARCHAR(100) NOT NULL
,   `age`        INTEGER
,   `hash`       CHAR(64)
,   `available`  BOOLEAN NOT NULL DEFAULT TRUE
,   `created_at` DATETIME DEFAULT CURRENT_TIMESTAMP
);
