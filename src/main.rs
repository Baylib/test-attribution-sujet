use chrono::{DateTime, Duration, Utc};
use rand::seq::SliceRandom;
use rand::Rng;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

mod affectation;
use crate::affectation::Student;
use crate::affectation::Subject;

fn main() -> std::io::Result<()> {
    let start: DateTime<Utc> = Utc::now() + Duration::hours(1);
    println!("start at : {}", start);
    let n_sub = 15; // number of subject
    let n_stud = 50000; // number of student
    let n_pref = 5; // number of preferences
    let max_capacity = 10000;
    let mut output: String = String::new();
    output.push_str(&format!(
        "Example  with\n\t{} random subjects with capacity 1-{},\n\t{} random students with {} random preferences and 0-20 random NOEs\n\n",
        n_sub, max_capacity, n_stud, n_pref
    ));
    // random generator
    let mut rng = &mut rand::thread_rng();
    // create random subjects
    let mut subjects = Vec::new();
    for n in 1..n_sub + 1 {
        let mut curr = Subject::new(n);
        curr.set_capacity(rng.gen_range(1..max_capacity));
        subjects.push(curr);
    }
    // write subject to ouput
    /* output.push_str("Random subjects :\n");
    for sub in &subjects {
        output.push_str(&format!("\t{:?}\n", sub))
    } */
    // create random student with random pref
    let mut students = Vec::new();
    for n in 1..n_stud + 1 {
        let mut curr = Student::new(n);
        let pref: Vec<Subject> = subjects
            .choose_multiple(&mut rng, n_pref)
            .cloned()
            .collect();
        for k in pref {
            curr.add_pref(&k);
        }
        curr.set_noe(rng.gen_range(0..20));
        students.push(curr);
    }
    // keep initial student vector;
    let init_students = students.clone();
    /* output.push_str("Random students (most preferred subject is last) :\n");
    for st in &students {
        output.push_str(&format!("\t{:?}\n", st))
    } */
    // affect student to subjects
    // output.push_str("Progress : \n");
    let mut not_affected = Vec::new();
    let mut in_assignment = Vec::<Student>::new();
    //students.shuffle(&mut rand::thread_rng());
    //students.sort_by(|a, b| b.noe.cmp(&a.noe));
    let mut count = 0;
    while students.len() > 0 {
        count += 1;
        if count % 1000 == 0 {
            println!("current count : {}", count);
        }
        let mut current = students.pop().unwrap();
        let pref_subject_id = current.preferences.pop().unwrap();
        let mut selected_sub: Vec<&mut Subject> = subjects
            .iter_mut()
            .filter(|x| x.id == pref_subject_id)
            .collect();
        let result_add = selected_sub[0].add_student_id(&current);
        match result_add {
            Some(x) => {
                if x == current.id {
                    if current.preferences.len() > 0 {
                        /* output.push_str(&format!(
                            "\tstudent {} was rejected from subject {}\n",
                            current.id, selected_sub[0].id
                        )); */
                        students.push(current);
                    } else {
                        /* output
                        .push_str(&format!("\tstudent {} was rejected from all\n", current.id)); */
                        not_affected.push(current)
                    }
                } else {
                    let rejected_student: Vec<Student> = in_assignment
                        .iter()
                        .cloned()
                        .filter(|a| a.id == x)
                        .collect();
                    if rejected_student[0].preferences.len() > 0 {
                        students.push(rejected_student[0].clone());
                    } else {
                        /* output.push_str(&format!(
                            "\tstudent {} was rejected from all\n",
                            rejected_student[0].id
                        )); */
                        not_affected.push(rejected_student[0].clone())
                    }
                    /* output.push_str(&format!(
                        "\tstudent {} took the place of student {} in subject {}\n",
                        current.id, rejected_student[0].id, selected_sub[0].id
                    )); */
                    let index = in_assignment
                        .iter()
                        .position(|r| r.id == rejected_student[0].id)
                        .unwrap();
                    in_assignment.remove(index);
                    in_assignment.push(current.clone());
                }
            }
            None => {
                in_assignment.push(current.clone());
                /* output.push_str(&format!(
                    "\tstudent {} was added to subject {}\n",
                    current.id, selected_sub[0].id
                )); */
            }
        }
    }

    // push results to output
    /* output.push_str("Subjects after assignments (id, noe) :\n");
    for sub in &subjects {
        output.push_str(&format!("\t{:?}\n", sub))
    } */

    output.push_str(&format!(
        "Students left unassigned {} , soit {:.2} % :\n",
        not_affected.len(),
        ((not_affected.len() as f32 * 100.) / init_students.len() as f32),
    ));
    for alone in not_affected {
        let x: Vec<&Student> = init_students.iter().filter(|a| a.id == alone.id).collect();
        output.push_str(&format!("\t{:?}\n", x))
    }

    // output.push_str("Students at the end :\n");
    let mut statistics = HashMap::new();
    for st in &in_assignment {
        let key = st.preferences.len();
        if statistics.contains_key(&key) {
            *statistics.get_mut(&key).unwrap() += 1;
        } else {
            statistics.insert(st.preferences.len(), 1);
        }
        // output.push_str(&format!("\t{:?}\n", st))
    }
    output.push_str("Statistics :\n");
    for (statkey, statvalue) in &statistics {
        output.push_str(&format!(
            "\tstudent with pref {} : {}\n",
            (n_pref - statkey),
            statvalue
        ));
    }
    let end: DateTime<Utc> = Utc::now() + Duration::hours(1);
    println!("end at : {}", end);
    let execution_time = end - start;
    println!("duration : {} seconds", execution_time.num_seconds());
    println!("while loop counter : {}", count);
    // push output to file
    let now: DateTime<Utc> = Utc::now() + Duration::hours(1);
    let today = format!("{}", now.format("%Y-%m%d-%H%M%S"));
    let filename = format!("./output/affectation_result_{}.txt", today);
    let mut file = File::create(filename)?;
    file.write_all(output.as_bytes())?;
    Ok(())
}
