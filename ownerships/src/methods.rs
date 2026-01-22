#[derive(Debug)]
struct MyTask {
    task_id: u32,
    task_name: String,
    task_type: String,
    daily: bool,
    weekly: bool,
    monthly: bool,
    total_daily_tasks: u32,
    total_daily_task_completed: u32,
}

impl MyTask {
    fn add_task(
        task_id: u32,
        task_name: String,
        task_type: String,
        daily: bool,
        weekly: bool,
        monthly: bool,
        total_daily_tasks: u32,
        total_daily_task_completed: u32,
    ) -> MyTask {
        return MyTask {
            task_id,
            task_name,
            task_type,
            daily,
            weekly,
            monthly,
            total_daily_tasks,
            total_daily_task_completed,
        };
    }

    fn daily_success_rate(&self) -> f32 {
        if self.daily {
            (self.total_daily_task_completed as f32 / self.total_daily_tasks as f32) * 100.0
        } else {
            0.0
        }
    }
}

pub fn run() {
    let task = MyTask::add_task(
        1,
        String::from("my first task"),
        String::from("urgent"),
        true,
        false,
        false,
        7,
        5,
    );

    println!("new daily task is {:#?}", task);

    println!(
        "your daily success rate is {:?}% ",
        task.daily_success_rate()
    );
}
