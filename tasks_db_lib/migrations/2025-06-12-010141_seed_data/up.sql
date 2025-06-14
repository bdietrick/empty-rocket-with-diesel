-- Users
INSERT INTO users (user_id, name, email, active) VALUES (1,'Alice', 'alice@example.com', 1);
INSERT INTO users (user_id, name, email, active) VALUES (2,'Bob', 'bob@example.com', 1);
INSERT INTO users (user_id, name, email, active) VALUES (3,'Charlie', 'charlie@example.com', 1);
INSERT INTO users (user_id, name, email, active) VALUES (4,'Diana', 'diana@example.com', 0);
INSERT INTO users (user_id, name, email, active) VALUES (5,'Eve', 'eve@example.com', 1);
INSERT INTO users (user_id, name, email, active) VALUES (6,'Frank', 'frank@example.com', 0);
INSERT INTO users (user_id, name, email, active) VALUES (7,'Grace', 'grace@example.com', 1);
INSERT INTO users (user_id, name, email, active) VALUES (8,'Heidi', 'heidi@example.com', 1);
INSERT INTO users (user_id, name, email, active) VALUES (9,'Ivan', 'ivan@example.com', 0);
INSERT INTO users (user_id, name, email, active) VALUES (10,'Judy', 'judy@example.com', 1);

-- Task Status
INSERT INTO task_statuses (task_status_id, status_name) VALUES (1, 'Not Started');
INSERT INTO task_statuses (task_status_id, status_name) VALUES (2, 'In Progress');
INSERT INTO task_statuses (task_status_id, status_name) VALUES (3, 'Completed');

-- Tasks
INSERT INTO tasks (task_id, task_name) VALUES (1, 'Write project proposal');
INSERT INTO tasks (task_id, task_name) VALUES (2, 'Design database schema');
INSERT INTO tasks (task_id, task_name) VALUES (3, 'Implement authentication');
INSERT INTO tasks (task_id, task_name) VALUES (4, 'Set up CI/CD pipeline');
INSERT INTO tasks (task_id, task_name) VALUES (5, 'Write unit tests');
INSERT INTO tasks (task_id, task_name) VALUES (6, 'Deploy to staging');
INSERT INTO tasks (task_id, task_name) VALUES (7, 'Review pull requests');
INSERT INTO tasks (task_id, task_name) VALUES (8, 'Update documentation');
INSERT INTO tasks (task_id, task_name) VALUES (9, 'Fix reported bugs');
INSERT INTO tasks (task_id, task_name) VALUES (10, 'Prepare release notes');

-- UserTasks
INSERT INTO user_tasks (user_id, task_id, task_status_id) VALUES (1, 2, 1);
INSERT INTO user_tasks (user_id, task_id, task_status_id) VALUES (1, 5, 3);
INSERT INTO user_tasks (user_id, task_id, task_status_id) VALUES (1, 7, 2);
INSERT INTO user_tasks (user_id, task_id, task_status_id) VALUES (1, 9, 2);
INSERT INTO user_tasks (user_id, task_id, task_status_id) VALUES (2, 1, 2);
INSERT INTO user_tasks (user_id, task_id, task_status_id) VALUES (2, 3, 1);
INSERT INTO user_tasks (user_id, task_id, task_status_id) VALUES (2, 6, 1);
INSERT INTO user_tasks (user_id, task_id, task_status_id) VALUES (2, 8, 3);
INSERT INTO user_tasks (user_id, task_id, task_status_id) VALUES (2, 10, 1);
INSERT INTO user_tasks (user_id, task_id, task_status_id) VALUES (3, 1, 1);
INSERT INTO user_tasks (user_id, task_id, task_status_id) VALUES (3, 4, 2);
INSERT INTO user_tasks (user_id, task_id, task_status_id) VALUES (3, 5, 3);
INSERT INTO user_tasks (user_id, task_id, task_status_id) VALUES (3, 8, 3);
INSERT INTO user_tasks (user_id, task_id, task_status_id) VALUES (3, 9, 2);
INSERT INTO user_tasks (user_id, task_id, task_status_id) VALUES (4, 2, 2);
INSERT INTO user_tasks (user_id, task_id, task_status_id) VALUES (4, 3, 1);
INSERT INTO user_tasks (user_id, task_id, task_status_id) VALUES (4, 6, 1);
INSERT INTO user_tasks (user_id, task_id, task_status_id) VALUES (4, 7, 2);
INSERT INTO user_tasks (user_id, task_id, task_status_id) VALUES (4, 10, 1);
INSERT INTO user_tasks (user_id, task_id, task_status_id) VALUES (5, 1, 1);
INSERT INTO user_tasks (user_id, task_id, task_status_id) VALUES (5, 2, 2);
INSERT INTO user_tasks (user_id, task_id, task_status_id) VALUES (5, 4, 2);
INSERT INTO user_tasks (user_id, task_id, task_status_id) VALUES (5, 6, 1);
INSERT INTO user_tasks (user_id, task_id, task_status_id) VALUES (5, 8, 3);
INSERT INTO user_tasks (user_id, task_id, task_status_id) VALUES (6, 3, 1);
INSERT INTO user_tasks (user_id, task_id, task_status_id) VALUES (6, 5, 3);
INSERT INTO user_tasks (user_id, task_id, task_status_id) VALUES (6, 7, 2);
INSERT INTO user_tasks (user_id, task_id, task_status_id) VALUES (6, 9, 2);
INSERT INTO user_tasks (user_id, task_id, task_status_id) VALUES (7, 1, 1);
INSERT INTO user_tasks (user_id, task_id, task_status_id) VALUES (7, 4, 2);
INSERT INTO user_tasks (user_id, task_id, task_status_id) VALUES (7, 6, 1);
INSERT INTO user_tasks (user_id, task_id, task_status_id) VALUES (7, 8, 3);
INSERT INTO user_tasks (user_id, task_id, task_status_id) VALUES (7, 10, 1);
INSERT INTO user_tasks (user_id, task_id, task_status_id) VALUES (8, 2, 2);
INSERT INTO user_tasks (user_id, task_id, task_status_id) VALUES (8, 3, 1);
INSERT INTO user_tasks (user_id, task_id, task_status_id) VALUES (8, 5, 3);
INSERT INTO user_tasks (user_id, task_id, task_status_id) VALUES (8, 7, 2);
INSERT INTO user_tasks (user_id, task_id, task_status_id) VALUES (8, 9, 2);
INSERT INTO user_tasks (user_id, task_id, task_status_id) VALUES (9, 1, 1);
INSERT INTO user_tasks (user_id, task_id, task_status_id) VALUES (9, 4, 2);
INSERT INTO user_tasks (user_id, task_id, task_status_id) VALUES (9, 6, 1);
INSERT INTO user_tasks (user_id, task_id, task_status_id) VALUES (9, 8, 3);
INSERT INTO user_tasks (user_id, task_id, task_status_id) VALUES (9, 10, 1);
INSERT INTO user_tasks (user_id, task_id, task_status_id) VALUES (10, 2, 2);
INSERT INTO user_tasks (user_id, task_id, task_status_id) VALUES (10, 3, 1);
INSERT INTO user_tasks (user_id, task_id, task_status_id) VALUES (10, 5, 3);
INSERT INTO user_tasks (user_id, task_id, task_status_id) VALUES (10, 7, 2);
INSERT INTO user_tasks (user_id, task_id, task_status_id) VALUES (10, 9, 2);






