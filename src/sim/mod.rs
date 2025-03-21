use mat_vec::Vector3;
use ObjectType::*;

pub struct World {
    pub objects: Vec<Object>,
}

pub enum ObjectType {
    Removed,
    Massive,
    Massless,
}

pub struct Object {
    pub pos: Vector3<f64>,
    pub vel: Vector3<f64>,
    pub mass: f64,
    pub class: ObjectType,
}

pub fn process_step(world: &mut World, delta_t: f64) {
    let mut accelerations = Vec::with_capacity(world.objects.capacity());
    for i in 0..world.objects.len() {
        //let mut acceleration = (0.0, 0.0);
        let obj = &world.objects[i];
        match obj.class {
            Massive | Massless => {
                let mut total_force = Vector3::new(0.0, 0.0, 0.0);
                for j in 0..world.objects.len() {
                    let obj_2 = &world.objects[j];
                    if j != i {
                        if let Massive = &obj_2.class {
                            let displacement = obj.pos - obj_2.pos;
                            let dist_sqr = displacement.x().powi(2) + displacement.y().powi(2);
                            //let dist = dist_sqr.sqrt();
                            let dir = -displacement.normalize();
                            total_force += dir * obj.mass * obj_2.mass * (1.0 / dist_sqr);
                        }
                    }
                }
                accelerations.push(total_force * (1.0 / obj.mass))
            }
            _ => (),
        }
    }
    let mut index = 0;
    for obj in &mut world.objects {
        match obj.class {
            Massive | Massless => {
                obj.vel += accelerations[index] * delta_t;
                obj.pos += obj.vel * delta_t;
            }
            _ => (),
        }
        index += 1
    }
}
