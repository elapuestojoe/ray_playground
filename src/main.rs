use r_ray::ray::Ray;
use r_vector::vector::Vector;
use r_vector::vector::VectorOperations;

fn hit_sphere(center: &Vector<f32>, radius: f32, ray: &Ray<f32>) -> f32 {
    let oc = ray.origin() - center;

    let a = ray.direction().dot(&ray.direction());
    let b = 2.0 * oc.dot(ray.direction());
    let c = oc.dot(&oc) - (radius * radius);
    let discriminant = b * b - 4.0 * a * c;
    if discriminant < 0.0 {
        return -1.0;
    }
    (-b - discriminant.sqrt()) / (2.0 * a)
}

fn color(ray: &Ray<f32>) -> Vector<f32> {
    let t = hit_sphere(&Vector::<f32>::new(0.0, 0.0, -1.0), 0.5, ray);
    if t > 0.0 {
        let normal = (&ray.point_at_time(t) - &Vector::<f32>::new(0.0, 0.0, -1.0)).unit_vector();
        return Vector::<f32>::new(normal.x() + 1.0, normal.y() + 1.0, normal.z() + 1.0) * 0.5;
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

            let ir = (255.99 * color.r()) as i32;
            let ig = (255.99 * color.g()) as i32;
            let ib = (255.99 * color.b()) as i32;

            println!("{} {} {}\n", ir, ig, ib);
        }
    }
}
