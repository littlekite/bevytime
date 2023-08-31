
use  bevy::prelude::*;
use bevy::window::WindowResolution;
use  chrono::Local;
use  mysql::prelude::*;
use  mysql::Pool; 
use std::process::Command;
use bevy_framepace::{FramepacePlugin, FramepaceSettings, Limiter};

fn main() {
    App::new()
    .add_plugins(DefaultPlugins.set(WindowPlugin {
            //设置窗口大小 1100*750
            primary_window: Some(Window {
                #[cfg(target_os = "windows")]
                position: WindowPosition::Centered(MonitorSelection::Primary), //窗口居中
                resolution: WindowResolution::new(500.0, 200.0),
                ..default()
            }),
            ..default()
        }),)
    .add_plugins(FramepacePlugin)
    .add_systems(Update, testsystems)
    .add_systems(Startup, testsystems1)
    .run();
}

#[derive(Component)]
struct EnableText;

fn testsystems(
    mut text: Query<&mut Text, With<EnableText>>,
){
    let dt = Local::now();
    let dt_string = dt.format("%Y-%m-%d %H:%M:%S").to_string();
    println!("当前时间{}",dt_string);
    text.single_mut().sections[0].value = dt_string;
    
}

fn testsystems1(
    mut commands: Commands,
    mut framepace: ResMut<FramepaceSettings>,   
){
    framepace.limiter = Limiter::from_framerate(30.0);

    let dt = Local::now();
    let dt_string = dt.format("%Y-%m-%d %H:%M:%S").to_string();
    println!("当前时间{}",dt_string);
    // 创建MySQL连接池
    /* 
    let pool = Pool::new("mysql://root:root@localhost:3306/kite").unwrap();

    // 从连接池中获取连接
    let mut conn = pool.get_conn().unwrap();

    // 执行SQL查询
    let query = "SELECT * FROM mytable";
    let result: Vec<(i32, String)> = conn.query_map(query, |(id, name)| (id, name)).unwrap();

    // 输出查询结果
    for (id, name) in result {
        println!("ID: {}, Name: {}", id, name);
    }
    */
    commands.spawn((Camera2dBundle {
        camera: Camera {
            order: 10,
            ..default()
        },
        ..default()
    },));
    // UI
    let style = TextStyle {
        font_size: 60.0,
        color: Color::WHITE,
        ..default()
    };
    commands.spawn((
        TextBundle::from_sections(vec![
            TextSection {
                value: "当前时间: ".to_string(),
                style: style.clone(),
            },
        ]),
        EnableText,
    ));
}


fn testsystems2(
    mut commands: Commands
){
    let output = Command::new("python").arg("t.py").output().expect("msg");
    // 检查执行结果
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        println!("Python script output: {}", stdout);
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        println!("Python script error: {}", stderr);
    }
}