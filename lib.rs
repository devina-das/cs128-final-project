enum Importance {
    Low, 
    Medium, 
    High,
}

enum DayOfWeek {
    Mon, 
    Tue, 
    Wed, 
    Thu, 
    Fri, 
    Sat, 
    Sun,
}

// I'm thinking time can be represented as 5.45 for 5:45
// We could bound it from 0.00 -> 23.59
// We would have to make sure the decimal part is like 0 -> 59
struct Task {
    id: i32,
    title: String,
    time: f64,
    importance: Importance,
    desc: String,
    status: bool,
}

// Tasks[0] -> (Mon) for example
pub struct List {
    next_id: i32,
    tasks: Vec<Vec<Task>>
}

impl Task {
    pub fn edit_title(&mut self, title: String) {
        self.title = title;
    }
    pub fn edit_time(&mut self, time: f64) {
        if !check_time(time) {
            // do smth
        }
        self.time = time;
    }
    pub fn edit_importance(&mut self, importance: Importance) {
        self.importance = importance;
    }
    pub fn edit_desc(&mut self, desc: String) {
        self.desc = desc;
    }
    pub fn mark_done(&mut self) {
        self.status = true;
    }
}

impl List {
    pub fn new() -> List {
        let days: Vec<Vec<Task>> = Vec::new();
        for _ in 0..7 {
            days.push(Vec::new());
        }
        List {
            next_id: 0,
            tasks: days,
        }
    } 
    pub fn add_task(&mut self, day: DayOfWeek, title: String, time: f64, importance: Importance, desc: String) {
        if !check_time(time) {
            // do smth
        }
        let new_task: Task = Task {
            id: self.next_id,
            title: title,
            time: time,
            importance: importance,
            desc: desc,
            status: false,
        };
        self.next_id += 1;
        let idx: i32 = match day {
            DayOfWeek::Mon => 0,
            DayOfWeek::Tue => 1,
            DayOfWeek::Wed => 2,
            DayOfWeek::Thu => 3,
            DayOfWeek::Fri => 4,
            DayOfWeek::Sat => 5,
            DayOfWeek::Sun => 6,
        };
        self.tasks[idx].push(new_task);
    }

    pub fn remove_task(&mut self, id: i32) {

    }

    pub fn edit_task(&mut self, id: i32) {

    }

    pub fn display(&mut self, id: i32) {

    }

    // moves all completed tasks to end of each day
    pub fn organize(&mut self) {

    }
}



// Kinda the logic for checking time, but I don't know where/how to implement.
fn check_time(time: f64) -> bool {
    let int: i32 = time.floor();
    let dec: f64 = time - int;
    int >= 0 && int < 24 && dec >= 0 && dec < 0.60; 
}