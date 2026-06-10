// base matrix
pub fn model_matrix() -> [[f32; 4];4]{
    [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 2.0, 1.0]
    ]
}

// view matrix, where and what the camera can see
pub fn view_matrix(position: &[f32; 3], direction: &[f32; 3], up: &[f32; 3]) -> [[f32; 4];4]{
    let f = {
        let f = direction;
        let len = f[0] * f[0] + f[1] * f[1] + f[2] * f[2];
        let len = len.sqrt();
        [f[0] / len, f[1] / len, f[2] / len]
    };

    let s = [up[1] * f[2] - up[2] * f[1],
             up[2] * f[0] - up[0] * f[2],
             up[0] * f[1] - up[1] * f[0]];

    let s_norm = {
        let len = s[0] * s[0] + s[1] * s[1] + s[2] * s[2];
        let len = len.sqrt();
        [s[0] / len, s[1] / len, s[2] / len]
    };

    let u = [f[1] * s_norm[2] - f[2] * s_norm[1],
             f[2] * s_norm[0] - f[0] * s_norm[2],
             f[0] * s_norm[1] - f[1] * s_norm[0]];

    let p = [-position[0] * s_norm[0] - position[1] * s_norm[1] - position[2] * s_norm[2],
             -position[0] * u[0] - position[1] * u[1] - position[2] * u[2],
             -position[0] * f[0] - position[1] * f[1] - position[2] * f[2]];

    [
        [s_norm[0], u[0], f[0], 0.0],
        [s_norm[1], u[1], f[1], 0.0],
        [s_norm[2], u[2], f[2], 0.0],
        [p[0],      p[1], p[2], 1.0],
    ]
}

// rotation matrix on all axis depending on a:yaw, b:pitch and g:roll
pub fn rotation_matrix((a,b,g):(f32, f32, f32)) -> [[f32; 4];4]{
    let ca = a.cos();
    let cb = b.cos();
    let cg = g.cos();

    let sa = a.sin();
    let sb = b.sin();
    let sg = g.sin();

    [
        [(ca*cb),            (sa*cb),            (-sb),   0.0],
        [(ca*sb*sg - sa*cg), (sa*sb*sg + ca*cg), (cb*sg), 0.0],
        [(ca*sb*cg + sa*sg), (sa*sb*cg - ca*sg), (cb*cg), 0.0],
        [0.0,                0.0,                1.0,     1.0],
    ]
}

// projection matrix, to make the gpu know what divide x and y for
pub fn projection_matrix((width, height): (u32, u32)) -> [[f32; 4]; 4]{
    let aspect_ratio = height as f32 / width as f32;

    const PI:f32 = 3.141592;
    let fov:f32 = 90.0; // degrees
    let f:f32 = 1.0 / (fov * 0.5 / 180.0 * PI ).tan(); // converting to radians

    let zfar:f32 = 1024.0; // how far the screen goes
    let znear:f32 = 0.1; // how close the screen is

    let q:f32 = zfar / (zfar - znear);

    [
        [aspect_ratio * f,  0.0,    0.0,    0.0],
        [0.0,               f,      0.0,    0.0],
        [0.0,               0.0,    q,      1.0],
        [0.0,               0.0,-znear * q, 0.0],
    ]
}

