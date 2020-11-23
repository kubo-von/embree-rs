#![allow(dead_code)]
extern crate cgmath;
extern crate embree;
extern crate support;

use cgmath::{Vector2,Vector3, Vector4, InnerSpace};
use embree::{Device, Geometry, IntersectContext, QuadMesh, Ray, RayHit, Scene, TriangleMesh, LinearCurve};
use support::Camera;

fn make_curve<'a>(device: &'a Device) -> Geometry<'a> {
    let mut curve = LinearCurve::unanimated(&device, 2, 3, 0);
    {
        let mut verts = curve.vertex_buffer.map();
        let mut ids = curve.index_buffer.map();
        let mut flags = curve.index_buffer.map();
        verts[0] = Vector4::new(-0.0, -0.0, -0.0, 1.0);
        verts[1] = Vector4::new(-0.0, 10.0, 0.0, 0.5);
        verts[2] = Vector4::new(-0.0, 15.0, 5.0, 0.25);
        ids[0] = 0;
        ids[1] = 1;
        flags[0] = 0; //not sure how these shoudl be set, https://www.embree.org/api.html#rtccurveflags , https://www.embree.org/api.html#rtc_geometry_type_curve
        flags[1] = 1; 

    }
    let mut curve_geo = Geometry::LinearCurve(curve);
    curve_geo.commit();
    curve_geo
}

fn make_ground_plane<'a>(device: &'a Device) -> Geometry<'a> {
    let mut mesh = QuadMesh::unanimated(device, 1, 4);
    {
        let mut verts = mesh.vertex_buffer.map();
        let mut quads = mesh.index_buffer.map();
        verts[0] = Vector4::new(-10.0, -2.0, -10.0, 0.0);
        verts[1] = Vector4::new(-10.0, -2.0, 10.0, 0.0);
        verts[2] = Vector4::new(10.0, -2.0, 10.0, 0.0);
        verts[3] = Vector4::new(10.0, -2.0, -10.0, 1.0);

        quads[0] = Vector4::new(0, 1, 2, 3);
    }
    let mut mesh = Geometry::Quad(mesh);
    mesh.commit();
    mesh
}

fn main() {
    let mut display = support::Display::new(512, 512, "curve geometry");
    let device = Device::new();
    let ground = make_ground_plane(&device);
    let curve = make_curve(&device);

    let mut scene = Scene::new(&device);
    let curve_id = scene.attach_geometry(curve);
    let ground_id = scene.attach_geometry(ground);
    let rtscene = scene.commit();

    let mut intersection_ctx = IntersectContext::coherent();

    display.run(|image, camera_pose, _| {
        for p in image.iter_mut() {
            *p = 0;
        }
        let img_dims = image.dimensions();
        let camera = Camera::look_dir(
            camera_pose.pos,
            camera_pose.dir,
            camera_pose.up,
            75.0,
            img_dims,
        );
        // Render the scene
        for j in 0..img_dims.1 {
            for i in 0..img_dims.0 {
                let dir = camera.ray_dir((i as f32 + 0.5, j as f32 + 0.5));
                let ray = Ray::new(camera.pos, dir);
                let mut ray_hit = RayHit::new(ray);
                rtscene.intersect(&mut intersection_ctx, &mut ray_hit);
                if ray_hit.hit.hit() {
                    let h = ray_hit.hit;
                    let mut p = image.get_pixel_mut(i, j);
                    
                    let color = Vector3::new(0.3, 0.3, 0.3);
                    let N = Vector3::new(h.Ng_x,h.Ng_y,h.Ng_z).normalize();
                    let uv = Vector3::new(h.u,h.v,0.0);

                    p[0] = ((uv.x/2.+0.5) * 255.0) as u8;
                    p[1] = ((uv.y/2.+0.5) * 255.0) as u8;
                    p[2] = (0.0) as u8;

/*                     p[0] = ((N.x/2. +0.0) * 255.0) as u8;
                    p[1] = ((N.y/2. +0.0) * 255.0) as u8;
                    p[2] = ((N.z/2. +0.0) *255.0) as u8; */

                    //p[0] = (color.x * 255.0) as u8;
                    //p[1] = (color.y * 255.0) as u8;
                    //p[2] = (color.z * 255.0) as u8;
                }
            }
        }
    });
}
