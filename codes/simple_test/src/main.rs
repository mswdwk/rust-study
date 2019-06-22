use std::collections::{HashMap,BTreeMap};

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
    
	let mut bmap = BTreeMap::new();
 // let student1 = Student { id:1,name:"name_1".to_owned()};
 let student2 = Student { id:3,name:"name_3".to_owned()};
 let student3 = Student { id:5,name:"name_5".to_owned()};
 let student4 = Student { id:2,name:"name_2".to_owned()};
 let student5 = Student { id:6,name:"name_6".to_owned()};
 let student6 = Student { id:4,name:"name_4".to_owned()};
	// bmap.insert(student1.id,student1);
	bmap.insert(student2.id,student2);
	bmap.insert(student3.id,student3);
	bmap.insert(student4.id,student4);
	bmap.insert(student5.id,student5);
	bmap.insert(student6.id,student6);

	for (id,stu) in &bmap {
	println!("id= {} student= {:?}",id,stu);
	}
	println!("student= {:?}",bmap.get(&2));
}
