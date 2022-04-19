use miniproject::{checkTime, merge_sort, print_table, Activity, GreedyActivitySelect};

fn main() {
    //Time should be in 24hr format
    //Defining Day 1 activities
    let mut day1_activities = vec![
        Activity::new(1, 9, 10, "COMP 342"), //Activity::new(group,start_time,finish_time,course_name)
        Activity::new(2, 9, 11, "COMP 302"),
        Activity::new(2, 14, 15, "COMP 312"),
        Activity::new(3, 11, 12, "COMP 302"),
        Activity::new(4, 13, 14, "COMP 441"),
        Activity::new(5, 15, 16, "COMP 201"),
        Activity::new(6, 15, 16, "COMP 112"),
        Activity::new(6, 10, 12, "COMP 101"),
    ];

    //Defining Day 2 activities
    let mut day2_activities = vec![
        Activity::new(1, 9, 10, "COMP 342"),
        Activity::new(2, 9, 10, "COMP 302"),
        Activity::new(3, 11, 12, "COMP 302"),
        Activity::new(1, 13, 14, "COMP 441"),
        Activity::new(5, 14, 15, "COMP 201"),
        Activity::new(6, 14, 16, "COMP 112"),
        Activity::new(6, 10, 11, "COMP 201"),
    ];

    //Defining Day 3 activities
    let mut day3_activities = vec![
        Activity::new(1, 9, 10, "COMP 342"),
        Activity::new(2, 9, 10, "COMP 302"),
        Activity::new(3, 11, 12, "COMP 302"),
        Activity::new(1, 13, 14, "COMP 441"),
        Activity::new(5, 14, 15, "COMP 201"),
        Activity::new(6, 14, 16, "COMP 112"),
        Activity::new(6, 10, 11, "COMP 201"),
    ];

    //Defining Day 4 activities
    let mut day4_activities = vec![
        Activity::new(1, 9, 10, "COMP 342"), //Activity::new(group,start_time,finish_time,course_name)
        Activity::new(2, 9, 11, "COMP 302"),
        Activity::new(2, 14, 15, "COMP 312"),
        Activity::new(3, 11, 12, "COMP 302"),
        Activity::new(4, 13, 14, "COMP 441"),
        Activity::new(5, 15, 16, "COMP 201"),
        Activity::new(6, 15, 16, "COMP 112"),
        Activity::new(6, 10, 12, "COMP 101"),
    ];

    //Defining Day 5 activities
    let mut day5_activities = vec![
        Activity::new(1, 9, 10, "COMP 342"),
        Activity::new(2, 9, 10, "COMP 302"),
        Activity::new(3, 11, 12, "COMP 302"),
        Activity::new(1, 13, 14, "COMP 441"),
        Activity::new(5, 14, 15, "COMP 201"),
        Activity::new(6, 14, 16, "COMP 112"),
        Activity::new(6, 10, 11, "COMP 201"),
    ];
    //Defining Day 6 activities
    let mut day6_activities = vec![
        Activity::new(1, 9, 10, "COMP 342"), //Activity::new(group,start_time,finish_time,course_name)
        Activity::new(2, 9, 11, "COMP 302"),
        Activity::new(2, 14, 15, "COMP 312"),
        Activity::new(3, 11, 12, "COMP 302"),
        Activity::new(4, 13, 14, "COMP 441"),
        Activity::new(5, 15, 16, "COMP 201"),
        Activity::new(6, 15, 16, "COMP 112"),
        Activity::new(6, 10, 12, "COMP 101"),
    ];

    //Placing all the activities in week
    let mut week = vec![
        day1_activities,
        day2_activities,
        day3_activities,
        day4_activities,
        day5_activities,
        day6_activities,
    ];

    //Going throuch each day activities and selecting the appropriate activities
    for (index, mut activities) in &mut week.iter_mut().enumerate() {
        //First of all sorting the activities in terms of finish_time in ascending order
        let length = activities.len();
        merge_sort(&mut activities, 0, length - 1);

        //After sorting the activities check the time for some constraints
        //Panics if constraint is not satisfied
        checkTime(&activities);
        //activities is selected for classroom 1 and remaining activities is selelcted  for classroom2 and so on to 5
        println!("\n\nDay :{}", index + 1);
        let (classroom_1, mut remaining_activity1) = GreedyActivitySelect(&mut activities);
        let (classroom_2, mut remaining_activity2) = GreedyActivitySelect(&mut remaining_activity1);
        let (classroom_3, mut remaining_activity3) = GreedyActivitySelect(&mut remaining_activity2);
        let (classroom_4, mut remaining_activity4) = GreedyActivitySelect(&mut remaining_activity3);
        let (classroom_5, mut remaining_activity5) = GreedyActivitySelect(&mut remaining_activity4);
        print_table(&classroom_1, "Classroom1");
        print_table(&classroom_2, "Classroom2");
        print_table(&classroom_3, "Classroom3");
        print_table(&classroom_4, "Classroom4");
        print_table(&classroom_5, "Classroom5");
    }
}
