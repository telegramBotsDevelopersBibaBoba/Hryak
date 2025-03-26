/*M!999999\- enable the sandbox mode */ 
-- MariaDB dump 10.19-11.7.2-MariaDB, for Linux (x86_64)
--
-- Host: localhost    Database: hryak
-- ------------------------------------------------------
-- Server version	11.7.2-MariaDB

/*!40101 SET @OLD_CHARACTER_SET_CLIENT=@@CHARACTER_SET_CLIENT */;
/*!40101 SET @OLD_CHARACTER_SET_RESULTS=@@CHARACTER_SET_RESULTS */;
/*!40101 SET @OLD_COLLATION_CONNECTION=@@COLLATION_CONNECTION */;
/*!40101 SET NAMES utf8mb4 */;
/*!40103 SET @OLD_TIME_ZONE=@@TIME_ZONE */;
/*!40103 SET TIME_ZONE='+00:00' */;
/*!40014 SET @OLD_UNIQUE_CHECKS=@@UNIQUE_CHECKS, UNIQUE_CHECKS=0 */;
/*!40014 SET @OLD_FOREIGN_KEY_CHECKS=@@FOREIGN_KEY_CHECKS, FOREIGN_KEY_CHECKS=0 */;
/*!40101 SET @OLD_SQL_MODE=@@SQL_MODE, SQL_MODE='NO_AUTO_VALUE_ON_ZERO' */;
/*M!100616 SET @OLD_NOTE_VERBOSITY=@@NOTE_VERBOSITY, NOTE_VERBOSITY=0 */;

--
-- Table structure for table `arcade`
--

DROP TABLE IF EXISTS `arcade`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `arcade` (
  `user_id` bigint(20) NOT NULL,
  `pigrace_last` timestamp NULL DEFAULT NULL,
  `treasurehunt_last` timestamp NULL DEFAULT NULL,
  PRIMARY KEY (`user_id`),
  CONSTRAINT `arcade_ibfk_1` FOREIGN KEY (`user_id`) REFERENCES `users` (`id`) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `arcade`
--

LOCK TABLES `arcade` WRITE;
/*!40000 ALTER TABLE `arcade` DISABLE KEYS */;
/*!40000 ALTER TABLE `arcade` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `bank`
--

DROP TABLE IF EXISTS `bank`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `bank` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT,
  `user_id` bigint(20) NOT NULL,
  `balance` double(10,2) DEFAULT 10.00,
  `daily_income` double(10,2) DEFAULT 10.00,
  `income_time` timestamp NULL DEFAULT NULL,
  PRIMARY KEY (`id`),
  KEY `idx_bank_user_id` (`user_id`),
  CONSTRAINT `fk_user_bank` FOREIGN KEY (`user_id`) REFERENCES `users` (`id`) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `bank`
--

LOCK TABLES `bank` WRITE;
/*!40000 ALTER TABLE `bank` DISABLE KEYS */;
/*!40000 ALTER TABLE `bank` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `duels`
--

DROP TABLE IF EXISTS `duels`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `duels` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT,
  `host_id` bigint(20) NOT NULL,
  `part_id` bigint(20) NOT NULL,
  `host_hp` double NOT NULL DEFAULT 100,
  `part_hp` double NOT NULL DEFAULT 100,
  `bid` double NOT NULL,
  `created_at` datetime NOT NULL DEFAULT current_timestamp(),
  `host_attack` double NOT NULL,
  `host_defense` double NOT NULL,
  `part_attack` double NOT NULL,
  `part_defense` double NOT NULL,
  PRIMARY KEY (`id`),
  UNIQUE KEY `unique_host` (`host_id`),
  KEY `part_id` (`part_id`),
  KEY `idx_duels_host_part` (`host_id`,`part_id`),
  CONSTRAINT `duels_ibfk_1` FOREIGN KEY (`host_id`) REFERENCES `users` (`id`) ON DELETE CASCADE,
  CONSTRAINT `duels_ibfk_2` FOREIGN KEY (`part_id`) REFERENCES `users` (`id`) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `duels`
--

LOCK TABLES `duels` WRITE;
/*!40000 ALTER TABLE `duels` DISABLE KEYS */;
/*!40000 ALTER TABLE `duels` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `inventory`
--

DROP TABLE IF EXISTS `inventory`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `inventory` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT,
  `user_id` bigint(20) DEFAULT NULL,
  `item_id` bigint(20) DEFAULT NULL,
  `usages` int(11) DEFAULT 1,
  PRIMARY KEY (`id`),
  KEY `item_id` (`item_id`),
  KEY `idx_inventory_user_item` (`user_id`,`item_id`),
  CONSTRAINT `inventory_ibfk_1` FOREIGN KEY (`user_id`) REFERENCES `users` (`id`),
  CONSTRAINT `inventory_ibfk_2` FOREIGN KEY (`item_id`) REFERENCES `shop_buffs` (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `inventory`
--

LOCK TABLES `inventory` WRITE;
/*!40000 ALTER TABLE `inventory` DISABLE KEYS */;
/*!40000 ALTER TABLE `inventory` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Temporary table structure for view `inventory_slot_view`
--

DROP TABLE IF EXISTS `inventory_slot_view`;
/*!50001 DROP VIEW IF EXISTS `inventory_slot_view`*/;
SET @saved_cs_client     = @@character_set_client;
SET character_set_client = utf8mb4;
/*!50001 CREATE VIEW `inventory_slot_view` AS SELECT
 1 AS `id`,
  1 AS `item_id`,
  1 AS `title`,
  1 AS `type`,
  1 AS `strength`,
  1 AS `usages`,
  1 AS `user_id` */;
SET character_set_client = @saved_cs_client;

--
-- Table structure for table `pigs`
--

DROP TABLE IF EXISTS `pigs`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pigs` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT,
  `user_id` bigint(20) NOT NULL,
  `weight` double(10,2) DEFAULT 50.00,
  `attack` double(10,2) DEFAULT 5.00,
  `defense` double(10,2) DEFAULT 5.00,
  `name` varchar(16) NOT NULL DEFAULT 'Unnamed',
  PRIMARY KEY (`id`),
  KEY `fk_user` (`user_id`),
  KEY `idx_pigs_user_id` (`user_id`),
  CONSTRAINT `fk_user` FOREIGN KEY (`user_id`) REFERENCES `users` (`id`) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `pigs`
--

LOCK TABLES `pigs` WRITE;
/*!40000 ALTER TABLE `pigs` DISABLE KEYS */;
/*!40000 ALTER TABLE `pigs` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `shop_buffs`
--

DROP TABLE IF EXISTS `shop_buffs`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `shop_buffs` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT,
  `title` varchar(255) NOT NULL,
  `price` double(10,2) NOT NULL,
  `description` text DEFAULT NULL,
  `usages` int(11) DEFAULT 1,
  `type` enum('attack','defense') NOT NULL,
  `strength` double(10,2) NOT NULL,
  PRIMARY KEY (`id`),
  KEY `idx_shop_buffs_id` (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=21 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `shop_buffs`
--

LOCK TABLES `shop_buffs` WRITE;
/*!40000 ALTER TABLE `shop_buffs` DISABLE KEYS */;
INSERT INTO `shop_buffs` VALUES
(1,'Малый эликсир атаки',5.00,'Увеличивает атаку на 0.5 (1 раз)',1,'attack',0.50),
(2,'Малый щит',5.00,'Увеличивает защиту на 0.5 (1 раз)',1,'defense',0.50),
(3,'Средний эликсир атаки',10.00,'Увеличивает атаку на 1.0 (1 раз)',1,'attack',1.00),
(4,'Средний щит',10.00,'Увеличивает защиту на 1.0 (1 раз)',1,'defense',1.00),
(5,'Большой эликсир атаки',15.00,'Увеличивает атаку на 1.5 (1 раз)',1,'attack',1.50),
(6,'Большой щит',15.00,'Увеличивает защиту на 1.5 (1 раз)',1,'defense',1.50),
(7,'Эпический эликсир атаки',20.00,'Увеличивает атаку на 2.0 (1 раз)',1,'attack',2.00),
(8,'Эпический щит',20.00,'Увеличивает защиту на 2.0 (1 раз)',1,'defense',2.00),
(9,'Легендарный эликсир',25.00,'Увеличивает атаку на 2.5 (1 раз)',1,'attack',2.50),
(10,'Легендарный щит',25.00,'Увеличивает защиту на 2.5 (1 раз)',1,'defense',2.50),
(11,'Малый эликсир атаки ',8.00,'Увеличивает атаку на 0.5 (2 раза)',2,'attack',0.50),
(12,'Малый щит ',8.00,'Увеличивает защиту на 0.5 (2 раза)',2,'defense',0.50),
(13,'Средний эликсир атаки ',15.00,'Увеличивает атаку на 1.0 (2 раза)',2,'attack',1.00),
(14,'Средний щит ',15.00,'Увеличивает защиту на 1.0 (2 раза)',2,'defense',1.00),
(15,'Большой эликсир атаки ',25.00,'Увеличивает атаку на 1.5 (2 раза)',2,'attack',1.50),
(16,'Большой щит ',25.00,'Увеличивает защиту на 1.5 (2 раза)',2,'defense',1.50),
(17,'Эпический эликсир атаки ',35.00,'Увеличивает атаку на 2.0 (2 раза)',2,'attack',2.00),
(18,'Эпический щит ',35.00,'Увеличивает защиту на 2.0 (2 раза)',2,'defense',2.00),
(19,'Легендарный эликсир ',45.00,'Увеличивает атаку на 2.5 (3 раза)',3,'attack',2.50),
(20,'Легендарный щит ',45.00,'Увеличивает защиту на 2.5 (3 раза)',3,'defense',2.50);
/*!40000 ALTER TABLE `shop_buffs` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `shop_food`
--

DROP TABLE IF EXISTS `shop_food`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `shop_food` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT,
  `title` varchar(255) NOT NULL,
  `price` double(10,2) NOT NULL,
  `description` text DEFAULT NULL,
  `nutrition` double(10,2) NOT NULL,
  PRIMARY KEY (`id`),
  KEY `idx_shop_food_id` (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=21 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `shop_food`
--

LOCK TABLES `shop_food` WRITE;
/*!40000 ALTER TABLE `shop_food` DISABLE KEYS */;
INSERT INTO `shop_food` VALUES
(1,'Свежая трава',1.00,'Простая трава для хряка',50.00),
(2,'Морковка',2.00,'Сочная морковка для роста',100.00),
(3,'Яблоко',3.00,'Сладкое яблоко для хряка',150.00),
(4,'Кукурузная каша',5.00,'Сытная каша для быстрого роста',250.00),
(5,'Отруби',4.00,'Питательные отруби для хряка',200.00),
(6,'Трюфели',10.00,'Роскошные трюфели для элитного хряка',500.00),
(7,'Арахис',6.00,'Хрустящий арахис для разнообразия',300.00),
(8,'Свиная отбивная',8.00,'Ироничный выбор для хряка',400.00),
(9,'Рыбные консервы',7.00,'Необычный, но питательный выбор',350.00),
(10,'Картофель',3.50,'Классический картофель для хряка',175.00),
(11,'Свекла',2.50,'Сладкая свекла для роста',125.00),
(12,'Овсянка',4.50,'Полезная овсянка для здоровья',225.00),
(13,'Специальный корм',15.00,'Высококачественный корм для роста',750.00),
(14,'Витаминный комплекс',20.00,'Комплекс витаминов для роста',1000.00),
(15,'Протеиновый коктейль',25.00,'Для хряков, стремящихся к форме',1250.00),
(16,'Золотое яблоко',50.00,'Легендарное яблоко для хряка',2500.00),
(17,'Магический гриб',40.00,'Гриб с магическими свойствами роста',2000.00),
(18,'Эликсир роста',30.00,'Таинственный эликсир для роста',1500.00),
(19,'Пицца',12.00,'Не самая здоровая, но вкусная еда',600.00),
(20,'Шоколадный торт',18.00,'Сладкий торт для особых случаев',900.00);
/*!40000 ALTER TABLE `shop_food` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `shop_improvements`
--

DROP TABLE IF EXISTS `shop_improvements`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `shop_improvements` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT,
  `title` varchar(255) NOT NULL,
  `price` double(10,2) NOT NULL,
  `description` text DEFAULT NULL,
  `type` enum('attack','defense','income') NOT NULL,
  `strength` double(10,2) NOT NULL,
  PRIMARY KEY (`id`),
  KEY `idx_shop_improvements_id` (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=21 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `shop_improvements`
--

LOCK TABLES `shop_improvements` WRITE;
/*!40000 ALTER TABLE `shop_improvements` DISABLE KEYS */;
INSERT INTO `shop_improvements` VALUES
(1,'Легкий удар',10.00,'Увеличивает атаку на 0.5','attack',0.50),
(2,'Броня из соломы',10.00,'Увеличивает защиту на 0.5','defense',0.50),
(3,'Маленький кошелек',15.00,'Увеличивает доход на 1.0','income',1.00),
(4,'Средний удар',20.00,'Увеличивает атаку на 1.0','attack',1.00),
(5,'Деревянная броня',20.00,'Увеличивает защиту на 1.0','defense',1.00),
(6,'Средний кошелек',25.00,'Увеличивает доход на 2.0','income',2.00),
(7,'Сильный удар',30.00,'Увеличивает атаку на 1.5','attack',1.50),
(8,'Каменная броня',30.00,'Увеличивает защиту на 1.5','defense',1.50),
(9,'Большой кошелек',35.00,'Увеличивает доход на 3.0','income',3.00),
(10,'Мощный удар',40.00,'Увеличивает атаку на 2.0','attack',2.00),
(11,'Железная броня',40.00,'Увеличивает защиту на 2.0','defense',2.00),
(12,'Золотой кошелек',45.00,'Увеличивает доход на 4.0','income',4.00),
(13,'Эпический удар',50.00,'Увеличивает атаку на 2.5','attack',2.50),
(14,'Мифриловая броня',50.00,'Увеличивает защиту на 2.5','defense',2.50),
(15,'Платиновый кошелек',55.00,'Увеличивает доход на 5.0','income',5.00),
(16,'Легендарный удар',60.00,'Увеличивает атаку на 3.0','attack',3.00),
(17,'Адамантитовая броня',60.00,'Увеличивает защиту на 3.0','defense',3.00),
(18,'Алмазный кошелек',65.00,'Увеличивает доход на 6.0','income',6.00),
(19,'Удар бога',70.00,'Увеличивает атаку на 3.5','attack',3.50),
(20,'Броня бога',70.00,'Увеличивает защиту на 3.5','defense',3.50);
/*!40000 ALTER TABLE `shop_improvements` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `users`
--

DROP TABLE IF EXISTS `users`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `users` (
  `id` bigint(20) NOT NULL,
  `username` varchar(64) DEFAULT NULL,
  `admin` tinyint(1) DEFAULT 0,
  PRIMARY KEY (`id`),
  KEY `idx_users_id` (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `users`
--

LOCK TABLES `users` WRITE;
/*!40000 ALTER TABLE `users` DISABLE KEYS */;
/*!40000 ALTER TABLE `users` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Final view structure for view `inventory_slot_view`
--

/*!50001 DROP VIEW IF EXISTS `inventory_slot_view`*/;
/*!50001 SET @saved_cs_client          = @@character_set_client */;
/*!50001 SET @saved_cs_results         = @@character_set_results */;
/*!50001 SET @saved_col_connection     = @@collation_connection */;
/*!50001 SET character_set_client      = utf8mb3 */;
/*!50001 SET character_set_results     = utf8mb3 */;
/*!50001 SET collation_connection      = utf8mb3_uca1400_ai_ci */;
/*!50001 CREATE ALGORITHM=UNDEFINED */
/*!50013 DEFINER=`klewy`@`localhost` SQL SECURITY DEFINER */
/*!50001 VIEW `inventory_slot_view` AS select `i`.`id` AS `id`,`i`.`item_id` AS `item_id`,`shop`.`title` AS `title`,`shop`.`type` AS `type`,`shop`.`strength` AS `strength`,`i`.`usages` AS `usages`,`i`.`user_id` AS `user_id` from (`inventory` `i` join `shop_buffs` `shop` on(`i`.`item_id` = `shop`.`id`)) */;
/*!50001 SET character_set_client      = @saved_cs_client */;
/*!50001 SET character_set_results     = @saved_cs_results */;
/*!50001 SET collation_connection      = @saved_col_connection */;
/*!40103 SET TIME_ZONE=@OLD_TIME_ZONE */;

/*!40101 SET SQL_MODE=@OLD_SQL_MODE */;
/*!40014 SET FOREIGN_KEY_CHECKS=@OLD_FOREIGN_KEY_CHECKS */;
/*!40014 SET UNIQUE_CHECKS=@OLD_UNIQUE_CHECKS */;
/*!40101 SET CHARACTER_SET_CLIENT=@OLD_CHARACTER_SET_CLIENT */;
/*!40101 SET CHARACTER_SET_RESULTS=@OLD_CHARACTER_SET_RESULTS */;
/*!40101 SET COLLATION_CONNECTION=@OLD_COLLATION_CONNECTION */;
/*M!100616 SET NOTE_VERBOSITY=@OLD_NOTE_VERBOSITY */;

-- Dump completed on 2025-03-18 16:26:20
