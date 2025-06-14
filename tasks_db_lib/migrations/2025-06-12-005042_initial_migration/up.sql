-- Your SQL goes here
CREATE TABLE `tasks`(
	`task_id` INTEGER NOT NULL PRIMARY KEY,
	`task_name` TEXT NOT NULL
);

CREATE TABLE `task_statuses`(
	`task_status_id` INTEGER NOT NULL PRIMARY KEY,
	`status_name` TEXT NOT NULL
);

CREATE TABLE `users`(
	`user_id` INTEGER NOT NULL PRIMARY KEY,
	`name` TEXT NOT NULL,
	`email` TEXT NOT NULL,
	`active` BOOL NOT NULL
);

CREATE TABLE `user_tasks`(
	`user_id` INTEGER NOT NULL,
	`task_id` INTEGER NOT NULL,
	`task_status_id` INTEGER NOT NULL,
	PRIMARY KEY(`user_id`, `task_id`),
	FOREIGN KEY (`user_id`) REFERENCES `users`(`user_id`),
	FOREIGN KEY (`task_id`) REFERENCES `tasks`(`task_id`),
	FOREIGN KEY (`task_status_id`) REFERENCES `task_statuses`(`task_status_id`)
);

