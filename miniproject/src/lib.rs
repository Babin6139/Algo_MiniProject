#![allow(unused)]

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
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
    let mut G1: i32 = 0;
    let mut G2: i32 = 0;
    let mut G3: i32 = 0;
    let mut G4: i32 = 0;
    let mut G5: i32 = 0;
    let mut G6: i32 = 0;

    for activity in activities {
        match activity.group {
            1 => {
                G1 = G1 + (activity.finish_time - activity.start_time);
            }
            2 => {
                G2 = G2 + (activity.finish_time - activity.start_time);
            }
            3 => {
                G3 = G3 + (activity.finish_time - activity.start_time);
            }
            4 => {
                G4 = G4 + (activity.finish_time - activity.start_time);
            }
            5 => {
                G5 = G5 + (activity.finish_time - activity.start_time);
            }
            6 => {
                G6 = G6 + (activity.finish_time - activity.start_time);
            }
            _ => {}
        }
    }
    if G1 > 4 || G2 > 4 || G3 > 4 || G4 > 4 || G5 > 4 || G6 > 4 {
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
            if L1[i].finish_time < R1[j].finish_time {
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
