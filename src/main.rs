use schedule::List;

fn main() {
    let mut new_list = List::new();
    new_list.display();

    let day = schedule::DayOfWeek::Mon;
    let title = String::from("Tue");
    let time = 05.45;
    let desc = String::from("Adding task");
    let title2 = String::from("Wed");
    let desc2 = String::from("Adding task2");
    new_list.add_task(day, title, time, desc);
    new_list.display();
    new_list.add_task(day, title2, time, desc2);
    new_list.display();
}
