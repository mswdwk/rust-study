use std::collections::HashMap;

#[derive(Debug)]
struct Student {
    pub id : i32,
    pub name: String
}

macro_rules! insert_map {
	($mapname:ident,$(($key:expr,$v:expr))*) => {
		$(
			let student = Student { id:$key,name:$v.to_owned()};
			$mapname.insert($key,student);
		)*
	}
}

fn display_slice(students: &[&Student]) {
    println!("students= {:?}",students );
}

fn main() {
    let mut students: HashMap<i32,Student> = HashMap::new();
    insert_map!(students,(1,"name_1"));
    insert_map!(students,(2,"name_2"));
    insert_map!(students,(3,"name_3"));
    insert_map!(students,(3,"name_3")(4,"name_4")(5,"name_5"));
    println!("{:?}",students );
    let students_array: Vec<_> = students.iter().map(|(_,s)|s).collect();
    display_slice(&students_array[..]);
}
