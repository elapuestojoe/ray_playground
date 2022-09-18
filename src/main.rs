use r_ray::camera::camera::Camera;
use r_ray::geometry::sphere::Sphere;
use r_ray::hittable::hittable::HitRecord;
use r_ray::hittable::hittable::Hittable;
use r_ray::hittable::hittable::HittableList;
use r_ray::ray::Ray;
use r_vector::vector::Vector;
use r_vector::vector::VectorOperations;
use rand::Rng;

fn get_color(ray: &Ray<f32>, world: &HittableList<f32>) -> Vector<f32> {
    let mut hit_record = HitRecord::<f32>::new();
    if world.hit(&ray, 0.0, f32::MAX, &mut hit_record) {
        return Vector::<f32>::new(
            hit_record.normal.x() + 1.0,
            hit_record.normal.y() + 1.0,
            hit_record.normal.z() + 1.0,
        ) * 0.5;
    }

    let unit_direction = ray.direction().unit_vector();
    let t = (&unit_direction.y() + 1.0) * 0.5;
    Vector::<f32>::new(1.0, 1.0, 1.0) * (1.0 - t) + (Vector::<f32>::new(0.5, 0.7, 1.0) * t)
}

fn main() {
    let mut rng = rand::thread_rng();
    let nx = 200;
    let ny = 100;
    let ns = 100;
    println!("P3\n{} {} \n255\n", nx, ny);

    let sphere_1 = Box::new(Sphere::<f32>::new(Vector::new(0.0, 0.0, -1.0), 0.5));
    let sphere_2 = Box::new(Sphere::<f32>::new(Vector::new(0.0, -100.5, -1.0), 100.0));
    let world = HittableList::new(vec![sphere_1, sphere_2]);

    let lower_left_corner = Vector::<f32>::new(-2.0, -1.0, -1.0);
    let horizontal = Vector::<f32>::new(4.0, 0.0, 0.0);
    let vertical = Vector::<f32>::new(0.0, 2.0, 0.0);
    let origin = Vector::<f32>::new(0.0, 0.0, 0.0);
    let camera = Camera::new(origin, lower_left_corner, horizontal, vertical);

    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut color = Vector::<f32>::new(0.0, 0.0, 0.0);
            for _ in 0..ns {
                let u = ((i as f32) + rng.gen_range(0.0f32, 1.0f32)) / (nx as f32);
                let v = ((j as f32) + rng.gen_range(0.0f32, 1.0f32)) / (ny as f32);
                let ray = camera.get_ray(u, v);
                color += get_color(&ray, &world);
            }
            color /= ns as f32;
            let ir = (255.99 * color.r()) as i32;
            let ig = (255.99 * color.g()) as i32;
            let ib = (255.99 * color.b()) as i32;

            println!("{} {} {}\n", ir, ig, ib);
        }
    }
}
