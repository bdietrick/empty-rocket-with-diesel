use diesel::prelude::*;
use diesel::SqliteConnection;  
use tasks_db_lib::*;           // schema::users
use tasks_db_lib::crud::CrudOperations;
use tasks_db_lib::models::{User, NewUser, Task, NewTask, UserTask, NewUserTask, TaskStatus, NewTaskStatus};

fn main() {
    println!("Requesting connection...");
    let mut connection =  establish_connection();
    println!("Received connection...");

    // show_users(&mut connection);
    // show_user_tasks(&mut connection);
    
    // Demonstrate User CRUD operations
    // Create
    let new_user = NewUser { name: "Test User", email: "testuser@example.com", active: true };
    let created_user = match User::create(&mut connection, new_user) {
        Ok(user) => { println!("Created user: {} (id: {})", user.name, user.user_id); Some(user) },
        Err(e) => { println!("Create failed: {}", e); None }
    };

    if let Some(user) = &created_user {
        let fetched = User::read(&mut connection,user.user_id).unwrap();
        println!("Fetched user: {:?}", fetched);
    }
    
    // Update
    if let Some(user) = &created_user {
        let updated_user = NewUser { name: "Updated User", email: "updated@example.com", active: false };
        let updated = User::update(&mut connection, user.user_id, updated_user).unwrap();
        println!("Updated user: {:?}", updated);
    }

    // Read all
    let all_users = User::read_all(&mut connection).unwrap();
    println!("All users ({}):", all_users.len());
    for user in &all_users {
        println!("- {} (id: {})", user.name, user.user_id);
    }
    
    // Delete
    if let Some(user) = &created_user {
        let deleted = User::delete(&mut connection, user.user_id).unwrap();
        println!("Deleted {} user(s) with id {}", deleted, user.user_id);
    }

// ************************************
// Tasks
//*************************************
    // Demonstrate Task CRUD operations
    // Create
    let new_task = NewTask { task_name: "Test Task"};
    let created_task = match Task::create(&mut connection, new_task) {
        Ok(task) => { println!("Created task: {} (id: {})", task.task_name, task.task_id); Some(task) },
        Err(e) => { println!("Task create failed: {}", e); None }
    };

    if let Some(task) = &created_task {
        let fetched = Task::read(&mut connection,task.task_id).unwrap();
        println!("Fetched task: {:?}", fetched);
    }
    
    // Update
    if let Some(task) = &created_task {
        let updated_task = NewTask { task_name: "Updated Task" };
        let updated = Task::update(&mut connection,task.task_id,updated_task ).unwrap();
        println!("Updated task: {:?}", updated);
    }

    // Read all
    let all_tasks = Task::read_all(&mut connection).unwrap();
    println!("All tasks ({}):", all_tasks.len());
    for task in &all_tasks {
        println!("- {} (id: {})", task.task_name, task.task_id);
    }
    
    // Delete
    if let Some(task) = &created_task {
        let deleted = Task::delete(&mut connection, task.task_id).unwrap();
        println!("Deleted {} task(s) with id {}", deleted, task.task_id);
    }

// ************************************
// UserTasks
//*************************************
   // Demonstrate User Tasks CRUD operations
    // Create
    let uid = created_user.as_ref().unwrap().user_id;
    let tid = created_task.as_ref().unwrap().task_id;
    let tsid = 1;

    let new_user_task = NewUserTask { user_id: uid, task_id: tid, task_status_id: tsid };
    let created_user_task = match UserTask::create(&mut connection, new_user_task) {
        Ok(user_task) => { println!("Created user_task: (user_id: {}, task_id: {})", user_task.user_id, user_task.task_id); Some(user_task) },
        Err(e) => { println!("Create failed: {}", e); None }
    };

    if let Some(user_task) = &created_user_task {
        let fetched = UserTask::read(&mut connection,(uid,tid)).unwrap();
        println!("Fetched user task: {:?}", fetched);
    }
    
    // Update
    if let Some(user_task) = &created_user_task {
            if let Some(task) = &created_task {
            let tid = created_task.as_ref().unwrap().task_id;
            let updated_user_task = NewUserTask {user_id: user_task.user_id,task_id: tid, task_status_id: 2 };
            let updated = UserTask::update(&mut connection, (uid,tid), updated_user_task).unwrap();
            println!("Updated user task: {:?}", updated);
        }
    }

    // Read all
    let all_user_tasks = UserTask::read_all(&mut connection).unwrap();
    println!("All users tasks ({}):", all_user_tasks.len());
    for user_task in &all_user_tasks {
        println!("- user_id: {} task_id: {})", user_task.user_id, user_task.task_id );
    }
    
    // Delete
    if let Some(user_task) = &created_user_task {
        let deleted = UserTask::delete(&mut connection, (user_task.user_id, user_task.task_id)).unwrap();
        println!("Deleted {} user task(s) with user_id: {} and task_id: {}", deleted, user_task.user_id,user_task.task_id);
    }

// ************************************
// Task Statuses
//*************************************
   // Demonstrate Task Status CRUD operations
    // Create
    let uid = created_user.as_ref().unwrap().user_id;
    let tid = created_task.as_ref().unwrap().task_id;
    let tsid = 1;

    let new_task_status = NewTaskStatus { status_name: "Cancel" };
    let created_task_status = match TaskStatus::create(&mut connection, new_task_status) {
        Ok(task_status) => { println!("Created task_status: {} (id: {})",task_status.status_name, task_status.task_status_id); Some(task_status) },
        Err(e) => { println!("Create failed: {}", e); None }
    };

    if let Some(task_status) = &created_task_status {
        let fetched = TaskStatus::read(&mut connection,task_status.task_status_id).unwrap();
        println!("Fetched task status: {:?}", fetched);
    }
    
    // Update
    if let Some(task_status) = &created_task_status {
            if let Some(task) = &created_task {
            let tid = created_task.unwrap().task_id;
            let updated_task_status = NewTaskStatus {status_name: "Cancelled"};
            let updated = TaskStatus::update(&mut connection, task_status.task_status_id, updated_task_status).unwrap();
            println!("Updated task status: {:?}", updated);
        }
    }

    // Read all
    let all_task_statuses = TaskStatus::read_all(&mut connection).unwrap();
    println!("All task statuses ({}):", all_task_statuses.len());
    for task_status in &all_task_statuses {
        println!("- Task Status: {:?} task_status_id: {})", task_status.status_name, task_status.task_status_id );
    }
    
    // Delete
    if let Some(task_status) = &created_task_status {
        let deleted = TaskStatus::delete(&mut connection, task_status.task_status_id).unwrap();
        println!("Deleted {} user task(s) with id {}", deleted, task_status.task_status_id);
    }


}


fn show_user_tasks(conn: &mut SqliteConnection) {
    
    use tasks_db_lib::schema::users::dsl as users_dsl;
    use tasks_db_lib::schema::user_tasks::dsl as ut_dsl;
    use tasks_db_lib::schema::tasks::dsl as tasks_dsl;
    use tasks_db_lib::schema::task_statuses::dsl as ts_dsl;
    use diesel::prelude::*;

    let results = users_dsl::users
        .inner_join(ut_dsl::user_tasks.on(users_dsl::user_id.eq(ut_dsl::user_id)))
        .inner_join(tasks_dsl::tasks.on(ut_dsl::task_id.eq(tasks_dsl::task_id)))
        .inner_join(ts_dsl::task_statuses.on(ut_dsl::task_status_id.eq(ts_dsl::task_status_id)))
        .select((users_dsl::name, tasks_dsl::task_name, ts_dsl::status_name))
        .load::<(String, String, String)>(conn)
        .expect("Error loading user tasks");

    println!("\nUser | Task | Status");
    println!("----------------------");
    for (user, task, status) in results {
        println!("{} | {} | {}", user, task, status);
    }
}

fn show_users(conn: &mut SqliteConnection) {
    use diesel::prelude::*;
    use tasks_db_lib::models::*;   // users
    use tasks_db_lib::schema::users::dsl::*; // id, name, email, active, star
    
    println!("Querying users...");
    let results = users
        .filter(active.eq(true))
        .limit(5)
        .select(User::as_select())
        .load(conn)
        .expect("Error loading users");
    
    println!("User query returned...");
    println!();
    println!("Displaying {} users", results.len());
    
    for user in results {
        println!("-----------");
        println!("Name: {}", user.name);
        println!("Email: {}", user.email);
        println!("Active: {}", user.active);
    }
    println!("-----------");

    }

