extern crate gl;
extern crate sdl2;
use std::f32;
pub mod render_gl;


const W:f32 = 1.0;
const H:f32 = 1.0;
const X0: f32 = -0.5;
const Y0: f32 = 0.9;
const N:usize = 100;
const DX:f32 = W/(N as f32);
const DY:f32 = H/(N as f32);
const K:f32 = 50.0;
const C:f32 = 0.05;
const DT:f32 = 0.1;
const G:f32 = 0.001;
const WIND:f32 = 0.001;

fn init_pos(pos: &mut Vec<f32>){
    for i in 0..N+1{
        for j in 0..N+1{
            /*
            V1--V3
            |  /
            | /
            V2
            */
            let idx = j*3 + i*(N+1)*3;
            pos[idx] = X0 + (i as f32)*DX;
            pos[idx+1] = Y0 - (j as f32)*DY;
            pos[idx+2] = 0.0;
        }
    }
}

fn pos_to_vert(pos: &Vec<f32>, verts : &mut Vec<f32>){
    for i in 0..N{
        for j in 0..N{
            /*
            V1--V3
            |  /
            | /
            V2
            */
            let idx = j*6*3 + i*N*6*3;
            // let p_num = j + i*(N+1)
            let p_idx = (j+i*(N+1))*3;
            // VERT 1
            verts[idx] = pos[p_idx];
            verts[idx+1] = pos[p_idx+1];
            verts[idx+2] = pos[p_idx+2];
            // VERT 2
            verts[idx+3] = pos[p_idx+(N+1)*3];
            verts[idx+4] = pos[p_idx+(N+1)*3+1];
            verts[idx+5] = pos[p_idx+(N+1)*3+2];
            // VERT 3
            verts[idx+6] = pos[p_idx+3];
            verts[idx+7] = pos[p_idx+3+1];
            verts[idx+8] = pos[p_idx+3+2];
            /*
                V4
               /|
              / |
            V5__V6
            */
            // VERT 4
            verts[idx+9] = pos[p_idx+3];
            verts[idx+10] = pos[p_idx+3+1];
            verts[idx+11] = pos[p_idx+3+2];
            // VERT 5
            verts[idx+12] = pos[p_idx+(N+1)*3];
            verts[idx+13] = pos[p_idx+(N+1)*3+1];
            verts[idx+14] = pos[p_idx+(N+1)*3+2];
            // VERT 6
            verts[idx+15] = pos[p_idx+(N+1)*3+3];
            verts[idx+16] = pos[p_idx+(N+1)*3+3+1];
            verts[idx+17] = pos[p_idx+(N+1)*3+3+2];
        }
    }
}

fn upd_pos(pos: &mut Vec<f32>,vel: &Vec<f32>,acc: &Vec<f32>){
    for i in 0..pos.len(){
        if ((i%(3*(N+1)))<3) {
            continue;
        }
        pos[i] = pos[i] + vel[i]*DT + 0.5*acc[i]*DT*DT;
    }
}

fn upd_vel(vel: &mut Vec<f32>, acc: &Vec<f32>, nacc: &Vec<f32>, click: f32){
    for i in 0..vel.len(){
        vel[i] = vel[i] + 0.5*DT*(acc[i]+nacc[i]);
        if i%3==0{
            vel[i] += click*0.01;
        }
    }
}

fn mk_acc(pos: &Vec<f32>,vel: &Vec<f32>,tick: f32, click: f32) -> Vec<f32>{
    let mut new_acc: Vec<f32> = vec![0.;3*(N+1)*(N+1)];
    for i in 0..N+1{
        for j in 0..N+1{
            let idx = j*3 + i*(N+1)*3;
            if (j<N){
                let dx = pos[idx+3] - pos[idx];
                let dy = pos[idx+3+1] - pos[idx+1];
                let dz = pos[idx+3+2] - pos[idx+2];
                let dr = dx.hypot(dy).hypot(dz);
                let kdl = K*(dr - DX);
                new_acc[idx] += kdl*dx/dr;
                new_acc[idx+1] += kdl*dy/dr;
                new_acc[idx+2] += kdl*dz/dr;
            }
            if (j>0){
                let dx = pos[idx-3] - pos[idx];
                let dy = pos[idx-3+1] - pos[idx+1];
                let dz = pos[idx-3+2] - pos[idx+2];
                let dr = dx.hypot(dy).hypot(dz);
                let kdl = K*(dr - DX);
                new_acc[idx] += kdl*dx/dr;
                new_acc[idx+1] += kdl*dy/dr;
                new_acc[idx+2] += kdl*dz/dr;
            }
            if (i<N){
                let dx = pos[idx+3*(N+1)] - pos[idx];
                let dy = pos[idx+3*(N+1)+1] - pos[idx+1];
                let dz = pos[idx+3*(N+1)+2] - pos[idx+2];
                let dr = dx.hypot(dy).hypot(dz);
                let kdl = K*(dr - DX);
                new_acc[idx] += kdl*dx/dr;
                new_acc[idx+1] += kdl*dy/dr;
                new_acc[idx+2] += kdl*dz/dr;
            }
            if (i>0){
                let dx = pos[idx-3*(N+1)] - pos[idx];
                let dy = pos[idx-3*(N+1)+1] - pos[idx+1];
                let dz = pos[idx-3*(N+1)+2] - pos[idx+2];
                let dr = dx.hypot(dy).hypot(dz);
                let kdl = K*(dr - DX);
                new_acc[idx] += kdl*dx/dr;
                new_acc[idx+1] += kdl*dy/dr;
                new_acc[idx+2] += kdl*dz/dr;
            }
            new_acc[idx] -= vel[idx]*C;
            new_acc[idx+1] -= vel[idx+1]*C;
            new_acc[idx+2] -= vel[idx+2]*C;
            new_acc[idx+1] -= G;
           // new_acc[idx+2] += WIND*click;
        }
    }
    return new_acc;
}

fn main() {
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();

    let gl_attr = video_subsystem.gl_attr();

    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 1);

    let window = video_subsystem
        .window("ClothSim - Iman Hosseini", 900, 700)
        .opengl()
        .resizable()
        .build()
        .unwrap();

    let _gl_context = window.gl_create_context().unwrap();
    let _gl =
        gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

    // set up shader program

    use std::ffi::CString;
    let vert_shader =
        render_gl::Shader::from_vert_source(&CString::new(include_str!("triangle.vert")).unwrap())
            .unwrap();

    let frag_shader =
        render_gl::Shader::from_frag_source(&CString::new(include_str!("triangle.frag")).unwrap())
            .unwrap();

    let shader_program = render_gl::Program::from_shaders(&[vert_shader, frag_shader]).unwrap();

    // set up vertex buffer object


    // set up vertices
    let mut verts: Vec<f32> = vec![0.0;6*3*N*N];
    let mut pos: Vec<f32> = vec![0.0;3*(N+1)*(N+1)];
    let mut vel: Vec<f32> = vec![0.0;3*(N+1)*(N+1)];
    let mut acc: Vec<f32> = vec![0.0;3*(N+1)*(N+1)];
    init_pos(&mut pos);
    pos_to_vert(&pos, &mut verts);
   

    let mut vertices: Vec<f32> = vec![-0.5, -0.5, 0.0, 0.5, -0.5, 0.0, 0.0, 0.5, 0.0];

    let mut vbo: gl::types::GLuint = 0;
    unsafe {
        gl::GenBuffers(1, &mut vbo);
    }

    unsafe {
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,                                                       // target
            (verts.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr, // size of data in bytes
            verts.as_ptr() as *const gl::types::GLvoid, // pointer to data
            gl::STATIC_DRAW,                               // usage
        );
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
    }

    // set up vertex array object

    let mut vao: gl::types::GLuint = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut vao);
    }

    unsafe {
        gl::BindVertexArray(vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::EnableVertexAttribArray(0); // this is "layout (location = 0)" in vertex shader
        gl::VertexAttribPointer(
            0,         // index of the generic vertex attribute ("layout (location = 0)")
            3,         // the number of components per generic vertex attribute
            gl::FLOAT, // data type
            gl::FALSE, // normalized (int-to-float conversion)
            (3 * std::mem::size_of::<f32>()) as gl::types::GLint, // stride (byte offset between consecutive attributes)
            std::ptr::null(),                                     // offset of the first component
        );
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);
    }

    // set up shared state for window

    unsafe {
        gl::Viewport(0, 0, 900, 700);
        gl::ClearColor(0.3, 0.3, 0.5, 1.0);
    }

    // main loop

    let mut event_pump = sdl.event_pump().unwrap();
    let mut tick: f32 = 0.;
    let mut click: f32 = 0.;
    'main: loop {
        tick = tick + DT;
        click = 0.;
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'main,
                sdl2::event::Event::MouseButtonDown { .. } =>  click = 1.0,
                _ => {}
            }
        }

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        // draw triangle
        // vertices[0] = (tick*0.1).sin()*0.1-0.5;
        upd_pos(&mut pos, &vel, &acc);
        let nacc:Vec<f32> = mk_acc(&pos,&vel,tick,click);
        upd_vel(&mut vel, &acc, &nacc,click);
        acc = nacc;
        pos_to_vert(&pos, &mut verts);
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,                                                       // target
                (verts.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr, // size of data in bytes
                verts.as_ptr() as *const gl::types::GLvoid, // pointer to data
                gl::STATIC_DRAW,                               // usage
            );
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }

        shader_program.set_used();
        unsafe {
            gl::BindVertexArray(vao);
            gl::DrawArrays(
                gl::TRIANGLES, // mode
                0,             // starting index in the enabled arrays
                6*(N as i32)*(N as i32),             // number of indices to be rendered
            );
        }

        window.gl_swap_window();
    }
}