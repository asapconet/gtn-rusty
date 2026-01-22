#[derive(Debug)]
enum TaskStatus {
    COMPLETED,
}

#[derive(Debug)]
enum TaskType {
    URGENT,
}

//this is more like a tuple enum
enum TaskAnalysis {
    id(u32),
    is_task_info { id: u32, task_type: TaskType },
    is_task_progress { id: u32, status: TaskStatus },
}

// basically enums are an advanced and less stressful way to represent data in Rust.
// The take struct like features where you can just name an emnum and pass the types it expects,
// kind of like nesting object types in .js

impl TaskAnalysis {
    fn show(&self) {
        if let TaskAnalysis::id(id) = self {
            println!("Task ID: {}", id);
        } else {
            println!("no Task ID");
        }
    }
}

pub fn run() {
    let new_task = TaskAnalysis::is_task_info {
        id: 1,
        task_type: TaskType::URGENT,
    };

    let general_progress = TaskAnalysis::is_task_progress {
        id: 1,
        status: TaskStatus::COMPLETED,
    };

    let show_task_id = TaskAnalysis::id(55);
    show_task_id.show();
}
