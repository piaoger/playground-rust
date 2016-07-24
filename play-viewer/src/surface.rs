
use glium;
use glium::backend::Facade;
use camera;
use std;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 3],
    color: [f32; 3],
    normal: [f32; 3],
}

implement_vertex!(Vertex, position, color, normal);

pub struct Surface {
    verts: glium::VertexBuffer<Vertex>,
    indices: glium::IndexBuffer<u32>,
    program: glium::Program,
    normal_program: glium::Program,
    model: [[f32; 4]; 4],
    color: [f32; 4],
}

impl Surface {
    pub fn new<F: Facade>(facade: &F,
                          verts: &[[f64; 3]],
                          faces: &[[u32; 3]],
                          normals: &[[f64; 3]])
                          -> Surface {

        // compute the bounding box
        let mut xmin = std::f64::MAX;
        let mut ymin = std::f64::MAX;
        let mut zmin = std::f64::MAX;
        let mut xmax = std::f64::MIN;
        let mut ymax = std::f64::MIN;
        let mut zmax = std::f64::MIN;

        for v in verts {
            xmin = xmin.min(v[0]);
            ymin = ymin.min(v[1]);
            zmin = zmin.min(v[2]);
            xmax = xmax.max(v[0]);
            ymax = ymax.max(v[1]);
            zmax = zmax.max(v[2]);
        }

        let cx = ((xmax + xmin) / 2.) as f32;
        let cy = ((ymax + ymin) / 2.) as f32;
        let cz = ((zmax + zmin) / 2.) as f32;
        let sx = ((xmax - xmin) / 2.) as f32;
        let sy = ((ymax - ymin) / 2.) as f32;
        let sz = ((zmax - zmin) / 2.) as f32;


        // center the object and scale to max size 1 in any direction
        let scale = 1. / sx.max(sy).max(sz);
        let model = [[scale, 0.0, 0.0, 0.0],
                     [0.0, scale, 0.0, 0.0],
                     [0.0, 0.0, scale, 0.0],
                     [-scale * cx, -scale * cy, -scale * cz, 1.0f32]];


        let verts = {

            let verts: Vec<_> = verts.iter()
                .zip(normals.iter())
                .map(|(v, n)| {
                    Vertex {
                        position: [v[0] as f32, v[1] as f32, v[2] as f32],
                        color: [1.0, 0.0, 0.0],
                        normal: [n[0] as f32, n[1] as f32, n[2] as f32],
                    }
                })
                .collect();
            glium::VertexBuffer::new(facade, &verts).unwrap()
        };

        let indices = {
            let mut buf = Vec::with_capacity(faces.len() * 3);
            for i in faces {
                buf.extend_from_slice(i);
            }
            glium::IndexBuffer::new(facade, glium::index::PrimitiveType::TrianglesList, &buf)
                .unwrap()
        };

        let vertex_shader_src = r#"
        #version 150

        in vec3 position;
        in vec3 normal;

        out vec3 v_normal;
        out vec3 v_position;

        uniform mat4 perspective;
        uniform mat4 view;
        uniform mat4 model;

        void main() {
            mat4 modelview = view * model;
            v_normal = transpose(inverse(mat3(modelview))) * normal;
            gl_Position = perspective * modelview * vec4(position, 1.0);
            v_position = gl_Position.xyz / gl_Position.w;
        }
    "#;

        let fragment_shader_src = r#"
        #version 150

        in vec3 v_normal;
        in vec3 v_position;

        out vec4 color;

        uniform vec4 surface_color;
        uniform vec3 u_light;

        const vec3 specular_color = vec3(1.0, 1.0, 1.0);

        void main() {
            float diffuse = max(dot(normalize(v_normal), normalize(u_light)), 0.0);

            vec3 camera_dir = normalize(v_position);
            vec3 half_direction = normalize(normalize(u_light) + camera_dir);
            float specular = pow(max(dot(half_direction, normalize(v_normal)), 0.0), 16.0);

            vec3 ambient_color = 0.2 * surface_color.rgb;
            vec3 diffuse_color = 0.6 * surface_color.rgb;
            color = vec4(ambient_color + diffuse * diffuse_color + specular * specular_color, surface_color.a);
        }
    "#;

        let program =
            glium::Program::from_source(facade, vertex_shader_src, fragment_shader_src, None)
                .unwrap();

        let normal_vertex_shader_src = r#"
    #version 150
    in vec3 position;
    in vec3 normal;

    out Vertex
    {
      vec3 normal;
    } vertex;

    void main()
    {
      gl_Position = vec4(position, 1.0);
      vertex.normal = normal;
    }
    "#;

        let normal_geometry_shader_src = r#"
    #version 150
    layout(triangles) in;

    // Three lines will be generated: 6 vertices
    layout(line_strip, max_vertices=6) out;

    uniform float normal_length;
    uniform mat4 perspective;
    uniform mat4 view;
    uniform mat4 model;

    in Vertex
    {
      vec3 normal;
    } vertex[];

    out vec4 vertex_color;

    void main()
    {
      int i;
      mat4 matrix = perspective * view * model;
      for(i=0; i < gl_in.length(); i++)
      {
        vec3 p = gl_in[i].gl_Position.xyz;
        vec3 n = normalize(vertex[i].normal);

        gl_Position =  matrix * vec4(p, 1.0);
        EmitVertex();

        gl_Position = matrix * vec4(p + n * normal_length, 1.0);
        EmitVertex();

        EndPrimitive();
      }
    }
            "#;

        let normal_fragment_shader_src = r#"
        #version 150


        const vec4 normal_color = vec4(0.0, 1.0, 0.0, 1.0);
        out vec4 color;

        void main() {
            color = normal_color;
        }
    "#;

        let normal_program = glium::Program::from_source(facade,
                                                         normal_vertex_shader_src,
                                                         normal_fragment_shader_src,
                                                         Some(normal_geometry_shader_src))
            .unwrap();
        Surface {
            verts: verts,
            indices: indices,
            program: program,
            normal_program: normal_program,
            model: model,
            color: [1., 0., 0., 1.]
        }
    }

    pub fn set_color<C>(&mut self, color: C) where C: Into<[f32; 4]> {
        self.color = color.into();
    }

    pub fn set_model_matrix<C>(&mut self, matrix: C) where C: Into<[[f32; 4];4]> {
        self.model = matrix.into();
    }

    pub fn model_matrix(&self) -> [[f32; 4];4] {
        self.model
    }


    pub fn draw<S: glium::Surface>(&self,
                                   target: &mut S,
                                   camera: &camera::CameraState,
                                   wireframe: bool,
                                   normals: bool)
                                   -> Result<(), glium::DrawError> {
        let light = [1., 1., 1.0f32];
        // let light = [1.4, 0.4, -0.7f32];

        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            polygon_mode: if wireframe {
                glium::draw_parameters::PolygonMode::Line
            } else {
                glium::draw_parameters::PolygonMode::Fill
            },
            line_width: Some(3.),
            // backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockWise,
            ..Default::default()
        };

        try!(target.draw(&self.verts,
                         &self.indices,
                         &self.program,
                         &uniform! { model: self.model,
                                view: camera.get_view(),
                                perspective: camera.get_perspective(),
                                u_light: light,
                                surface_color: self.color, },
                         &params));
        if normals {

            try!(target.draw(&self.verts,
                             &self.indices,
                             &self.normal_program,
                             &uniform! { model: self.model,
                                view: camera.get_view(),
                                perspective: camera.get_perspective(),
                                normal_length: 1f32 },
                             &params));
        }

        Ok(())

    }
}
