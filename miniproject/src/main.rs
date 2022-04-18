use miniproject::{checkTime, merge_sort, Activity, GreedyActivitySelect};

fn main() {
    let mut day1_activities = vec![
        Activity::new(1, 9, 10, "COMP 342"),
        Activity::new(2, 9, 10, "COMP 302"),
        Activity::new(3, 11, 12, "COMP 302"),
        Activity::new(4, 1, 2, "COMP 441"),
        Activity::new(5, 3, 4, "COMP 201"),
        Activity::new(6, 3, 4, "COMP 112"),
    ];
    let mut day2_activities = vec![
        Activity::new(1, 9, 10, "COMP 342"),
        Activity::new(2, 9, 10, "COMP 302"),
        Activity::new(3, 11, 12, "COMP 302"),
        Activity::new(4, 1, 2, "COMP 441"),
        Activity::new(5, 3, 4, "COMP 201"),
        Activity::new(6, 3, 4, "COMP 112"),
    ];
    let mut day3_activities = vec![
        Activity::new(1, 9, 10, "COMP 342"),
        Activity::new(2, 9, 10, "COMP 302"),
        Activity::new(3, 11, 12, "COMP 302"),
        Activity::new(4, 1, 2, "COMP 441"),
        Activity::new(5, 3, 4, "COMP 201"),
        Activity::new(6, 3, 4, "COMP 112"),
    ];
    let mut day4_activities = vec![
        Activity::new(1, 9, 10, "COMP 342"),
        Activity::new(2, 9, 10, "COMP 302"),
        Activity::new(3, 11, 12, "COMP 302"),
        Activity::new(4, 1, 2, "COMP 441"),
        Activity::new(5, 3, 4, "COMP 201"),
        Activity::new(6, 3, 4, "COMP 112"),
    ];
    let mut day5_activities = vec![
        Activity::new(1, 9, 10, "COMP 342"),
        Activity::new(2, 9, 10, "COMP 302"),
        Activity::new(3, 11, 12, "COMP 302"),
        Activity::new(4, 1, 2, "COMP 441"),
        Activity::new(5, 3, 4, "COMP 201"),
        Activity::new(6, 3, 4, "COMP 112"),
    ];
    let mut day6_activities = vec![
        Activity::new(1, 9, 10, "COMP 342"),
        Activity::new(2, 9, 10, "COMP 302"),
        Activity::new(3, 9, 10, "COMP 302"),
        Activity::new(4, 9, 10, "COMP 302"),
        Activity::new(5, 9, 12, "COMP 302"),
        Activity::new(3, 11, 12, "COMP 302"),
        Activity::new(4, 1, 2, "COMP 441"),
        Activity::new(5, 3, 4, "COMP 201"),
        Activity::new(6, 3, 4, "COMP 112"),
    ];
    let mut day7_activities = vec![
        Activity::new(3, 11, 12, "COMP 302"),
        Activity::new(1, 9, 10, "COMP 342"),
        Activity::new(4, 13, 14, "COMP 441"),
        Activity::new(5, 15, 16, "COMP 201"),
        Activity::new(2, 9, 10, "COMP 302"),
        Activity::new(6, 15, 16, "COMP 112"),
    ];
    let length = day7_activities.len();

    let mut week = vec![
        day1_activities,
        day2_activities,
        day3_activities,
        day4_activities,
        day5_activities,
        day6_activities,
        day7_activities,
    ];
    for mut activities in week {
        checkTime(&activities);
        merge_sort(&mut activities, 0, length - 1);
        let (classroom_1, mut remaining_activity1) = GreedyActivitySelect(&mut activities);
        println!("Classroom 1: {:#?}", classroom_1);
        let (classroom_2, mut remaining_activity2) = GreedyActivitySelect(&mut remaining_activity1);
        println!("Classroom 2: {:#?}", classroom_2);
        let (classroom_3, mut remaining_activity3) = GreedyActivitySelect(&mut remaining_activity2);
        println!("Classroom 3: {:#?}", classroom_1);
        let (classroom_4, mut remaining_activity4) = GreedyActivitySelect(&mut remaining_activity3);
        println!("Classroom 4: {:#?}", classroom_2);
        let (classroom_5, mut remaining_activity5) = GreedyActivitySelect(&mut remaining_activity4);
        println!("Classroom 5: {:#?}", classroom_5);
    }
}
