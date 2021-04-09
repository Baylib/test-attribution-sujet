#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct Student {
    pub id: u32,
    pub preferences: Vec<u32>,
    pub noe: u32,
}

#[derive(Debug, Clone)]
pub struct Subject {
    pub id: u32,
    initial_capacity: u32,
    capacity: u32,
    assigned_students: Vec<(u32, u32)>,
}

impl Student {
    pub fn new(id: u32) -> Student {
        Student {
            id,
            preferences: Vec::new(),
            noe: 0,
        }
    }

    pub fn add_pref(&mut self, subject: &Subject) {
        self.preferences.push(subject.id.clone())
    }

    pub fn set_noe(&mut self, noe: u32) {
        self.noe = noe
    }
}

impl Subject {
    pub fn new(id: u32) -> Subject {
        Subject {
            id,
            initial_capacity: 1,
            capacity: 1,
            assigned_students: Vec::new(),
        }
    }

    pub fn add_student_id(&mut self, student: &Student) -> Option<u32> {
        if self.capacity >= 1 {
            self.capacity = self.capacity - 1;
            self.assigned_students.push((student.id, student.noe));
            return None;
        } else {
            let mut new_affectation = Vec::new();
            let mut rejected_student = (student.id, student.noe);
            for s in &self.assigned_students {
                if s.1 < rejected_student.1 {
                    new_affectation.push(rejected_student);
                    rejected_student = (s.0, s.1);
                } else {
                    new_affectation.push(s.clone());
                }
            }
            self.assigned_students = new_affectation;
            return Some(rejected_student.0);
        }
    }

    pub fn set_capacity(&mut self, capacity: u32) {
        self.capacity = capacity;
        self.initial_capacity = capacity;
    }
}
