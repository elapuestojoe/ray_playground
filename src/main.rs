use r_ray::ray::Ray;
use r_vector::vector::Vector;
use r_vector::vector::VectorOperations;

fn hit_sphere(center: &Vector<f32>, radius: f32, ray: &Ray<f32>) -> bool {
    let oc = ray.origin() - center;

    let a = ray.direction().dot(&ray.direction());
    let b = 2.0 * oc.dot(ray.direction());
    let c = oc.dot(&oc) - (radius * radius);
    let discriminant = b * b - 4.0 * a * c;
    discriminant > 0.0
}

fn color(ray: &Ray<f32>) -> Vector<f32> {
    if hit_sphere(&Vector::<f32>::new(0.0, 0.0, -1.0), 0.5, ray) {
        return Vector::<f32>::new(1.0, 0.0, 0.0);
    }
    let unit_direction = ray.direction().unit_vector();
    let t = (&unit_direction.y() + 1.0) * 0.5;
    Vector::<f32>::new(1.0, 1.0, 1.0) * (1.0 - t) + (Vector::<f32>::new(0.5, 0.7, 1.0) * t)
}

fn main() {
    let nx = 200;
    let ny = 100;
    println!("P3\n{} {} \n255\n", nx, ny);

    let lower_left_corner = Vector::<f32>::new(-2.0, -1.0, -1.0);
    let horizontal = Vector::<f32>::new(4.0, 0.0, 0.0);
    let vertical = Vector::<f32>::new(0.0, 2.0, 0.0);
    let origin = Vector::<f32>::new(0.0, 0.0, 0.0);

    for j in (0..ny).rev() {
        for i in 0..nx {
            let u = (i as f32) / (nx as f32);
            let v = (j as f32) / (ny as f32);

            let ray = Ray::<f32>::new(
                origin.clone(),
                &lower_left_corner + &(&horizontal * u) + (&vertical * v),
            );
            let color = color(&ray);

            let ir = (255.99 * color.r()).floor() as i32;
            let ig = (255.99 * color.g()).floor() as i32;
            let ib = (255.99 * color.b()).floor() as i32;

            println!("{} {} {}\n", ir, ig, ib);
        }
    }
}
