#![allow(unused)]
use cli_table::{format::Justify, print_stdout, Cell, Style, Table};

#[derive(Clone, PartialEq, Debug, Copy)]
pub struct Activity {
    pub group: i32,
    pub start_time: i32,
    pub finish_time: i32,
    pub course_name: &'static str,
}

impl Activity {
    pub fn new(group: i32, start_time: i32, finish_time: i32, course_name: &'static str) -> Self {
        Self {
            group,
            start_time,
            finish_time,
            course_name,
        }
    }
}
pub fn GreedyActivitySelect(mut activity: &mut [Activity]) -> (Vec<Activity>, Vec<Activity>) {
    let n: usize = activity.len();
    if n != 0 {
        let mut remaining_activity: Vec<Activity> = Vec::new();
        let mut A = Vec::new();
        A.push(activity[0]);
        let mut k: usize = 0;
        let mut m: usize = 1;
        while m < n {
            if activity[m].start_time >= activity[k].finish_time {
                A.push(activity[m]);
                k = m;
            } else {
                remaining_activity.push(activity[m]);
            }
            m += 1;
        }
        (A, remaining_activity)
    } else {
        (vec![], vec![])
    }
}

pub fn checkTime(activities: &[Activity]) {
    let mut g1: i32 = 0;
    let mut g2: i32 = 0;
    let mut g3: i32 = 0;
    let mut g4: i32 = 0;
    let mut g5: i32 = 0;
    let mut g6: i32 = 0;
    let mut g1_prevactivity: Activity = Activity::new(1, 0, 0, "dummy");
    let mut g2_prevactivity: Activity = Activity::new(2, 0, 0, "dummy");
    let mut g3_prevactivity: Activity = Activity::new(3, 0, 0, "dummy");
    let mut g4_prevactivity: Activity = Activity::new(4, 0, 0, "dummy");
    let mut g5_prevactivity: Activity = Activity::new(5, 0, 0, "dummy");
    let mut g6_prevactivity: Activity = Activity::new(6, 0, 0, "dummy");

    for activity in activities {
        if activity.start_time < activity.finish_time {
            match activity.group {
                1 => {
                    if (g1_prevactivity.finish_time > activity.start_time
                        && g1_prevactivity.start_time != 0)
                    {
                        panic!("Single group has colliding times");
                    } else {
                        g1_prevactivity.finish_time = activity.finish_time;
                        g1_prevactivity.start_time = activity.start_time;
                    }
                    if (activity.finish_time - activity.start_time <= 2) {
                        g1 = g1 + (activity.finish_time - activity.start_time);
                    } else {
                        panic!("Single group has more the two hours continuos lecture");
                    }
                }
                2 => {
                    if (g2_prevactivity.finish_time > activity.start_time
                        && g2_prevactivity.start_time != 0)
                    {
                        panic!("Single group has colliding times");
                    } else {
                        g2_prevactivity.finish_time = activity.finish_time;
                        g2_prevactivity.start_time = activity.start_time;
                    }
                    if (activity.finish_time - activity.start_time <= 2) {
                        g2 = g2 + (activity.finish_time - activity.start_time);
                    } else {
                        panic!("Single group has more the two hours continuos lecture");
                    }
                }
                3 => {
                    if (g3_prevactivity.finish_time > activity.start_time
                        && g3_prevactivity.start_time != 0)
                    {
                        panic!("Single group has colliding times");
                    } else {
                        g3_prevactivity.finish_time = activity.finish_time;
                        g3_prevactivity.start_time = activity.start_time;
                    }
                    if (activity.finish_time - activity.start_time <= 2) {
                        g3 = g3 + (activity.finish_time - activity.start_time);
                    } else {
                        panic!("Single group has more the two hours continuos lecture");
                    }
                }
                4 => {
                    if (g4_prevactivity.finish_time > activity.start_time
                        && g4_prevactivity.start_time != 0)
                    {
                        panic!("Single group has colliding times");
                    } else {
                        g4_prevactivity.finish_time = activity.finish_time;
                        g4_prevactivity.start_time = activity.start_time;
                    }
                    if (activity.finish_time - activity.start_time <= 2) {
                        g4 = g4 + (activity.finish_time - activity.start_time);
                    } else {
                        panic!("Single group has more the two hours continuos lecture");
                    }
                }
                5 => {
                    if (g5_prevactivity.finish_time > activity.start_time
                        && g5_prevactivity.start_time != 0)
                    {
                        panic!("Single group has colliding times");
                    } else {
                        g5_prevactivity.finish_time = activity.finish_time;
                        g5_prevactivity.start_time = activity.start_time;
                    }
                    if (activity.finish_time - activity.start_time <= 2) {
                        g5 = g5 + (activity.finish_time - activity.start_time);
                    } else {
                        panic!("Single group has more the two hours continuos lecture");
                    }
                }
                6 => {
                    if (g6_prevactivity.finish_time > activity.start_time
                        && g6_prevactivity.start_time != 0)
                    {
                        panic!("Single group has colliding times");
                    } else {
                        g6_prevactivity.finish_time = activity.finish_time;
                        g6_prevactivity.start_time = activity.start_time;
                    }
                    if (activity.finish_time - activity.start_time <= 2) {
                        g6 = g6 + (activity.finish_time - activity.start_time);
                    } else {
                        panic!("Single group has more the two hours continuos lecture");
                    }
                }
                _ => {}
            }
        } else {
            panic!("Finish time is not greater than start time");
        }
    }
    if g1 > 4 || g2 > 4 || g3 > 4 || g4 > 4 || g5 > 4 || g6 > 4 {
        panic!("One of the groups contain more than 4 hours lecture");
    }
}

pub fn merge_sort(unsorted_data: &mut [Activity], p: usize, r: usize) {
    if p < r {
        let q = (p + r) / 2;
        merge_sort(unsorted_data, p, q);
        merge_sort(unsorted_data, q + 1, r);
        merge(unsorted_data, p, q, r);
    }
}

pub fn merge(unsorted_data: &mut [Activity], p: usize, q: usize, r: usize) -> () {
    let L1 = unsorted_data[p..q + 1].to_vec();
    let R1 = unsorted_data[q + 1..r + 1].to_vec();
    let (mut i, mut j) = (0, 0);
    for k in p..r + 1 {
        if i >= L1.len() {
            unsorted_data[k] = R1[j];
            j = j + 1;
        } else if j >= R1.len() {
            unsorted_data[k] = L1[i];
            i = i + 1;
        } else {
            if L1[i].finish_time <= R1[j].finish_time {
                unsorted_data[k] = L1[i];
                i = i + 1;
            } else {
                unsorted_data[k] = R1[j];
                j = j + 1;
            }
        }
    }
    ()
}

pub fn print_table(mut activity: &[Activity], class: &str) {
    println!("\n {}", class);
    for activity in activity {
        let table = vec![vec![format!(
            "{}-{} \n{}\nGroup {}",
            activity.start_time, activity.finish_time, activity.course_name, activity.group
        )
        .cell()
        .justify(Justify::Center)]]
        .table()
        .title(vec!["Activity".cell().bold(true)])
        .bold(true);
        assert!(print_stdout(table).is_ok());
    }
}
