#[derive(Debug)]
enum TaskStatus {
    COMPLETED,
}

#[derive(Debug)]
enum TaskType {
    URGENT,
}

//this is more like a tuple enum
#[derive(Debug)]
enum TaskAnalysis {
    is_task_info {
        id: Option<u32>,
        task_type: TaskType,
    },
    is_task_progress {
        id: Option<u32>,
        status: TaskStatus,
    },
    is_task_analysis {
        id: Option<u32>,
        task_type: TaskType,
        status: Option<TaskStatus>,
    },
}

// basically enums are an advanced and less stressful way to represent data in Rust.
// The take struct like features where you can just name an emnum and pass the types it expects,
// kind of like nesting object types in .js

// this should check which of the supplied task information is complete or not
impl TaskAnalysis {
    fn task_data_complete(&self) -> bool {
        match self {
            TaskAnalysis::is_task_info { id, task_type } => id.is_some(),
            TaskAnalysis::is_task_progress { id, status } => id.is_some(),
            TaskAnalysis::is_task_analysis {
                id,
                task_type,
                status,
            } => id.is_some() && status.is_some(),
        }
    }

    fn task_data_incomplete(&self) -> bool {
        !self.task_data_complete()
    }
}

pub fn run() {
    let new_task = TaskAnalysis::is_task_info {
        id: None,
        task_type: TaskType::URGENT,
    };

    let general_progress = TaskAnalysis::is_task_progress {
        id: Some(34),
        status: TaskStatus::COMPLETED,
    };

    //HOW TO USE OPTION from the method to check if task info is complete
    if new_task.task_data_complete() {
        println!("New task info is complete");
    } else if general_progress.task_data_complete() {
        println!("General task progress information is ready");
    } else if new_task.task_data_incomplete() {
        println!("New task analysis information is incomplete");
    } else {
        println!("Either Task progress or analysis information is incomplete");
    };
}
