
// Declare struct car
struct Car {
    color: String, 
    transmission: Transmision,
    convertible: bool,
    mileage: u32
}

#[derive(PartialEq, Debug)]
enum Transmision {
    // todo!("Fix enum definition so code compile")
    Manual, 
    SemiAuto,
    Automatic
}


fn car_factory(color: String, transmission: Transmision, convertible: bool) -> Car{
    let car:Car =  Car{
        color: color, 
        transmission: transmission, 
        convertible: convertible,
        mileage: 0
    };

    return car;
}


fn main() {
    let mut car = car_factory(String::from("Red"), Transmision::Manual, false);
    println!("Car 1 = {}, {:?} transmision, convertible: {}, mileage: {}\n", car.color, car.transmission, car.convertible, car.mileage);

    car = car_factory(String::from("Red"), Transmision::SemiAuto, false);
    println!("Car 1 = {}, {:?} transmision, convertible: {}, mileage: {}\n", car.color, car.transmission, car.convertible, car.mileage);

    car = car_factory(String::from("Red"), Transmision::Automatic, false);
    println!("Car 1 = {}, {:?} transmision, convertible: {}, mileage: {}\n", car.color, car.transmission, car.convertible, car.mileage);
    
}
