use r_float::Float;
use r_ray::camera::camera::Camera;
use r_ray::geometry::sphere::Sphere;
use r_ray::hittable::hittable::HitRecord;
use r_ray::hittable::hittable::Hittable;
use r_ray::hittable::hittable::HittableList;
use r_ray::material::materials_impl;
use r_ray::ray::Ray;
use r_vector::vector::Vector;
use r_vector::vector::VectorOperations;

fn get_color(ray: &Ray<f32>, world: &mut HittableList<f32>, depth: f32) -> Vector<f32> {
    let mut hit_record = HitRecord::<f32>::new();
    let closest_hit = world.hit(ray, 0.001, f32::MAX, &mut hit_record);
    if closest_hit.is_some() {
        let v_blank = Vector::new(0f32, 0f32, 0f32);
        let mut scattered = Ray::new(v_blank.clone(), v_blank.clone());
        let mut attenuation = v_blank.clone();
        if depth < 50f32
            && closest_hit
                .unwrap()
                .scatter(ray, &hit_record, &mut attenuation, &mut scattered)
        {
            return attenuation * get_color(&mut scattered, world, depth + 1.0);
        }
        return Vector::new(0f32, 0f32, 0f32);
    }
    let unit_direction = ray.direction().unit_vector();
    let t = (unit_direction.x() + 1.0) * 0.5;
    (Vector::new(1f32, 1f32, 1f32) * (1.0 - t)) + (Vector::new(0.5, 0.7, 1.0) * t)
}

fn main() {
    let nx = 200;
    let ny = 100;
    let ns = 100;
    println!("P3\n{} {} \n255\n", nx, ny);

    let sphere_1 = Sphere::new(
        Vector::new(0f32, 0f32, -1f32),
        0.5,
        materials_impl::Lambertian::new(Vector::new(0.8, 0.3, 0.3)),
    );
    let sphere_2 = Sphere::new(
        Vector::new(0f32, -100.5, -1f32),
        100f32,
        materials_impl::Lambertian::new(Vector::new(0.8, 0.8, 0.0)),
    );
    let sphere_3 = Sphere::new(
        Vector::new(1f32, 0f32, -1.2f32),
        0.5,
        materials_impl::Metal::new(Vector::new(0.8, 0.6, 0.2)),
    );
    let sphere_4 = Sphere::new(
        Vector::new(-1f32, 0f32, -1f32),
        0.5,
        materials_impl::Metal::new(Vector::new(0.8, 0.8, 0.8)),
    );
    let mut world = HittableList::new(vec![
        Box::new(sphere_1),
        Box::new(sphere_2),
        Box::new(sphere_3),
        Box::new(sphere_4),
    ]);

    let lower_left_corner = Vector::<f32>::new(-2.5, -1.0, -1.0);
    let horizontal = Vector::<f32>::new(4.0, 0.0, 0.0);
    let vertical = Vector::<f32>::new(0.0, 2.0, 0.0);
    let origin = Vector::<f32>::new(0.0, 0.0, 0.0);
    let camera = Camera::new(origin, lower_left_corner, horizontal, vertical);

    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut color = Vector::new(0f32, 0f32, 0f32);
            for _ in 0..ns {
                let mut f32_rand: f32 = Float::random();
                let f32_i: f32 = Float::from_i32(i);
                let f32_nx: f32 = Float::from_i32(nx);
                let u: f32 = (f32_i + f32_rand) / f32_nx;

                f32_rand = Float::random();
                let f32_j: f32 = Float::from_i32(j);
                let f32_ny: f32 = Float::from_i32(ny);
                let v: f32 = (f32_j + f32_rand) / f32_ny;

                let ray = camera.get_ray(u, v);
                color += get_color(&ray, &mut world, 0f32);
            }
            color /= ns as f32;
            color = Vector::new(color.r().sqrt(), color.g().sqrt(), color.b().sqrt());
            let ir = (255.99 * color.r()) as i32;
            let ig = (255.99 * color.g()) as i32;
            let ib = (255.99 * color.b()) as i32;

            println!("{} {} {}\n", ir, ig, ib);
        }
    }
}
