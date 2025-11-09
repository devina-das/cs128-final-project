use std::collections::HashMap;

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
pub enum DayOfWeek {
    Mon, Tue, Wed, Thu, Fri, Sat, Sun
}

// time can be repped as 23.59 for 23:59
struct Task {
    title: String,
    time: f64,
    desc: String,
}

pub struct List {
    next_id: usize,
    schedule: HashMap<DayOfWeek, HashMap<usize, Task>>
}

impl Task {
    pub fn display(&self){
        println!("{} @ {}", self.title, self.time);
        println!("{}", self.desc);
    }
}

impl List {
    pub fn new() -> List {
        let mut week: HashMap<DayOfWeek, HashMap<usize, Task>> = HashMap::new();
        week.insert(DayOfWeek::Mon, HashMap::new());
        week.insert(DayOfWeek::Tue, HashMap::new());
        week.insert(DayOfWeek::Wed, HashMap::new());
        week.insert(DayOfWeek::Thu, HashMap::new());
        week.insert(DayOfWeek::Fri, HashMap::new());
        week.insert(DayOfWeek::Sat, HashMap::new());
        week.insert(DayOfWeek::Sun, HashMap::new());

        List {
            next_id : 0,
            schedule: week
        }
    }

    pub fn add_task(&mut self, day: DayOfWeek, title: String, time: f64, desc: String) {
        if !check_time(time) {
            println!("invalid time");
            return;
        }
        let new_task = Task {title, time, desc};
        self.schedule.get_mut(&day).unwrap().insert(self.next_id, new_task);
        self.next_id += 1;
    }

    pub fn remove_task(&mut self, target: usize) {
        for (_, tasks) in self.schedule.iter_mut() {
            if tasks.remove(&target).is_some() {
                return;
            }
        }
    }

    pub fn edit_task() {
        todo!();
    }

    // This is broken I won't lie gang.
    pub fn display(&self) {
        println!("/////////////////////////////////////////");
        for (day, tasks) in &self.schedule {
            println!("{}", to_string(*day));
            for (id, task) in tasks {
                print!("#{} - ", id);
                task.display();
            }
        }
    }
}

fn check_time(time: f64) -> bool {
    let int = time.floor();
    let dec = time - int;
    int >= 0.00 && int < 24.00 && dec >= 0.00 && dec < 0.60 
}

fn to_string(day: DayOfWeek) -> String {
    let day_string = match day {
        DayOfWeek::Mon => String::from("Mon"),
        DayOfWeek::Tue => String::from("Tue"),
        DayOfWeek::Wed => String::from("Wed"),
        DayOfWeek::Thu => String::from("Thu"),
        DayOfWeek::Fri => String::from("Fri"),
        DayOfWeek::Sat => String::from("Sat"),
        DayOfWeek::Sun => String::from("Sun"),
    };
    day_string
}