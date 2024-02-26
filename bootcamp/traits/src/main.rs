// here Paint is supertrait for Vehicle
// any type implementing Vehicle trait must implement Paint trait
trait Vehicle: Paint + AnotherTrait {
    // associated function
    fn get_default_color() -> String {
        "block".to_owned()
    }
}

trait AnotherTrait {}

trait Park {
    fn park(&self);
}

trait Paint {
    fn paint(&self, color: String) {
        println!("painting object: {color}");
    }
}

struct VehicleInfo {
    make: String,
    model: String,
    year: u16,
}
struct Car {
    info: VehicleInfo,
}

impl Park for Car {
    fn park(&self) {
        println!("parking car!");
    }
}

impl Paint for Car {}

struct Truck {
    info: VehicleInfo,
}

impl Truck {
    fn unload(&self) {
        println!("unloading truck!");
    }
}

impl Park for Truck {
    fn park(&self) {
        println!("parking truck!");
    }
}

impl Paint for Truck {}

struct House {}

impl Paint for House {
    fn paint(&self, color: String) {
        println!("painting house: {color}");
    }
}

fn main() {
    let car = Car {
        info: VehicleInfo {
            make: "Honda".to_owned(),
            model: "Civic".to_owned(),
            year: 1995,
        },
    };

    let house = House {};
    let object = create_paintable_object();
    let object1 = create_paintable_object_condition(false);

    let paintable_objects: Vec<&dyn Paint> = vec![&car, &house];

    paint_red(&car);
    paint_red(&house);
    paint_red(&object);
    paint_vehicle_red(&car);
    paint_red1(object1.as_ref());
}

fn paint_red1(object: &dyn Paint) {
    object.paint("red".to_owned());
}

fn paint_red<T: Paint>(object: &T) {
    object.paint("red".to_owned());
}

fn paint_red2(object: &impl Paint) {
    object.paint("red".to_owned());
}

fn paint_red3<T>(object: &T)
where
    T: Paint,
{
    object.paint("red".to_owned());
}

fn paint_vehicle_red<T>(vehicle: &T)
where
    T: Park + Paint,
{
    vehicle.paint("red".to_owned());
}

fn create_paintable_object() -> impl Paint {
    House {}
}

fn create_paintable_object_condition(vehicle: bool) -> Box<dyn Paint> {
    if vehicle {
        Box::new(House {})
    } else {
        Box::new(Car {
            info: VehicleInfo {
                make: "BMW".to_owned(),
                model: "i5".to_owned(),
                year: 1999,
            }
        })
    }
}
