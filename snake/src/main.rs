


use std::{ thread::sleep, time::Duration,fs::File,io::{ErrorKind,Read}};
use bevy::prelude::*;
use rand;

struct MoveTimer(Timer);
struct Snake{
    direction:i8,
    parts:Vec<(f32,f32)>,
    speed:f32
}
struct Player;
struct Food;
struct TailDir{dir:usize}
struct Scoreboard{
    score:usize,
}
struct FrameTime{
    period:u64
}
struct TextTag;
struct GameOver{
    over:bool
}
struct OnDeath{ded:bool}
enum Collider{
    Snake,
    Solid
}

fn main() {
    let mut defvec:Vec<(f32,f32)>=Vec::new();
    for i in 0..3{
        defvec.push((32.0*(i as f32),0.0));
    }
    App::build().add_resource(WindowDescriptor{
        title: "Snake".to_string(),
        width: 800.0,
        height:800.0,
        ..Default::default()
    })
    .add_resource(FrameTime{period:frm_time()})
    .add_resource(Scoreboard{score:0})
    .add_resource(MoveTimer(Timer::new(Duration::from_millis(195. as u64),true)))
    .add_resource(Snake{direction:2,parts:defvec,speed:32.0})
    .add_resource(GameOver{over:false})
    .add_resource(TailDir{dir:2})
    .add_resource(OnDeath{ded:false})
    .add_plugins(DefaultPlugins)
    .add_startup_system(setup.system())
    .add_system(collision.system())
    .add_system(dir_snake.system())
    .add_system(move_snake.system())
    .add_system(game_over.system())
    .add_system(update_size.system())
    .run();
}

fn setup(
    commands: &mut Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    snake:Res<Snake>,
    assets: Res<AssetServer>
){
    commands
    .spawn(CameraUiBundle::default())
    .spawn(TextBundle{
        text: Text{value: "Score:".to_string(),
        font: assets.load("FiraSans-Bold.ttf"),
        style:TextStyle{
            font_size:30.0,
            color:Color::WHITE,
            ..Default::default()},..Default::default()},
        transform: Transform::from_translation(Vec3::new(0.0,0.0,0.0)),
        ..Default::default()
    })
    .with(TextTag)
        .spawn(Camera2dBundle::default());
        //snake
        for part in &snake.parts{
            commands.spawn(SpriteBundle{
                material:materials.add(Color::rgb(0.5, 0.2, 0.2).into()),
                transform: Transform::from_translation(Vec3::new(part.0,part.1,0.0)),
                sprite: Sprite::new(Vec2::new(32.0,32.0)),
                ..Default::default()
            }).with(Player)
            .with(Collider::Snake);
        }
        //food
        commands.spawn(SpriteBundle{
            material:materials.add(Color::rgb(0.2,0.2,0.2).into()),
            transform: Transform::from_translation(Vec3::new(64.0,64.0,0.0)),
            sprite: Sprite::new(Vec2::new(24.0,24.0)),
            ..Default::default()
        }).with(Food);
        //rigth wall
        commands
        .spawn(SpriteBundle{
            material:materials.add(Color::rgb(0.1, 0.1, 0.1).into()),
            transform:Transform::from_translation(Vec3::new(384.0,0.0,0.0)),
            sprite: Sprite::new(Vec2::new(32.0,736.0)),
            ..Default::default()
        })
        .with(Collider::Solid)
        //left wall
        .spawn(SpriteBundle{
            material:materials.add(Color::rgb(0.1, 0.1, 0.1).into()),
            transform: Transform::from_translation(Vec3::new(-384.0,0.0,0.0)),
            sprite: Sprite::new(Vec2::new(32.0,736.0)),
            ..Default::default()
        })
        .with(Collider::Solid)
        //top wall
        .spawn(SpriteBundle{
            material:materials.add(Color::rgb(0.1, 0.1, 0.1).into()),
            transform: Transform::from_translation(Vec3::new(0.0,384.0,0.0)),
            sprite: Sprite::new(Vec2::new(800.0,32.0)),
            ..Default::default()
        })
        .with(Collider::Solid)
        //bottom wall
        .spawn(SpriteBundle{
            material:materials.add(Color::rgb(0.1, 0.1, 0.1).into()),
            transform: Transform::from_translation(Vec3::new(0.0,-384.0,0.0)),
            sprite: Sprite::new(Vec2::new(800.0,32.0)),
            ..Default::default()
        })
        .with(Collider::Solid)
        ;

               
}


fn dir_snake(
    keyboard_input : Res<Input<KeyCode>>,
    mut snake: ResMut<Snake>,
    gameover : ResMut<GameOver>,

){  
    
    if gameover.over{
        //Causes "thread panicked while panickin. aborting." and I am too tired to debug
        //if keyboard_input.pressed(KeyCode::R){
        //    let mut defvec:Vec<(f32,f32)>=Vec::new();
        //    for i in 0..3{
        //        defvec.push((32.0*(i as f32),0.0));
        //    }
        //    score.score=0;
        //    commands
        //    .despawn(snake_ent.iter_mut().next().unwrap());
        //    tail_dir.dir=2;
        //    snake.parts=defvec;
        //    snake.direction=2;
        //    for part in snake.parts.clone(){
        //        commands.spawn(SpriteBundle{
        //            material:materials.add(Color::rgb(0.5, 0.2, 0.2).into()),
        //            transform: Transform::from_translation(Vec3::new(part.0,part.1,0.0)),
        //            sprite: Sprite::new(Vec2::new(32.0,32.0)),
        //            ..Default::default()
        //        }).with(Player)
        //        .with(Collider::Snake);
        //    }
        //    gameover.over=false;
        //    dead.ded=false;
        //}
        return;
    } 
        
    if keyboard_input.pressed(KeyCode::A){
        if snake.direction != 0{
        snake.direction=2;}
    }
    else if keyboard_input.pressed(KeyCode::D){
        if snake.direction != 2{
            snake.direction=0;}
    }
    else if keyboard_input.pressed(KeyCode::W){
        if snake.direction != 1{
            snake.direction=3;}
    }
    else if keyboard_input.pressed(KeyCode::S){
        if snake.direction != 3{
            snake.direction=1;}
    }
}

fn move_snake(
    //time : Res<Time>,
    //mut timer: ResMut<MoveTimer>,
    mut snake: ResMut<Snake>,
    mut act_snake: Query<&mut Transform,With<Player>>,
    gameover : Res<GameOver>,
    mut tail_dir: ResMut<TailDir>
    
){
   //if !timer.0.tick(time.delta_seconds()).finished()|| gameover.over{
   //    return;
   //}

   if gameover.over{
       return
   }
    let temp=snake.parts.clone();
    match snake.direction {
        0=> snake.parts[0].0+=snake.speed,
        1=>snake.parts[0].1-=snake.speed,
        2=>snake.parts[0].0-=snake.speed,
        3=>snake.parts[0].1+=snake.speed,
        _=>return
    }
    let mut val:usize=1;
    let mut tail= snake.parts.clone()[snake.parts.len()-1];
    for _i in 1..snake.parts.len(){
        snake.parts[val]=temp[val-1];
        val+=1;
    }
    tail.0 -= snake.parts.clone()[snake.parts.len()-1].0;
    tail.1 -= snake.parts.clone()[snake.parts.len()-1].1;

    if tail.0 == -32.0{
        tail_dir.dir=0;
    }else if tail.0 ==32.0{
        tail_dir.dir=2;
    }else if tail.1==-32.0{
        tail_dir.dir=3;
    }else if tail.1==32.0{
        tail_dir.dir=1;
    }
    let mut i=0;
    for mut part in act_snake.iter_mut(){
        part.translation= Vec3::new(snake.parts[i].0,snake.parts[i].1,0.0);
        i+=1;
    }
    
    
}
    


fn collision(
    mut food: Query<&mut Transform,With<Food>>,
    snake_coord: Res<Snake>,
    period: Res<FrameTime>,
    mut score: ResMut<Scoreboard>,
    mut gameover:ResMut<GameOver>
){
    if gameover.over{
        return;
    }
    sleep(std::time::Duration::from_millis(period.period));
    let mut food_coord= food.iter_mut().next().unwrap();
    for part in snake_coord.parts.clone(){
        if part.0 == food_coord.translation.x &&   part.1 == food_coord.translation.y{
            score.score+=1;
            food_coord.translation.x= -320.0 + 32.0*((rand::random::<u32>() % 7)+(rand::random::<u32>() % 7)+(rand::random::<u32>() % 7)) as f32;
            food_coord.translation.y= -320.0 + 32.0*((rand::random::<u32>() % 7)+(rand::random::<u32>() % 7)+(rand::random::<u32>() % 7)) as f32;
        }
    }
    
    let mut i=0;
    for part in snake_coord.parts.clone(){
        if snake_coord.parts.clone()[i].0+400.0 <=20.0 ||  snake_coord.parts.clone()[i].0+400.0 >=780.0 || snake_coord.parts.clone()[i].1+400.0 <=20.0 ||snake_coord.parts.clone()[i].1+400.0 >=780.0{
            gameover.over=true;
    
        }
        let mut j=0;
        for other in snake_coord.parts.clone(){
            if j!=i{
                if part==other{
                    gameover.over=true;
                }
            }
            j+=1;
        }
        i+=1;
    }
    
    
}

fn game_over(
    commands:&mut Commands,
    gameover:Res<GameOver>,
    mut textbnd : Query<&mut Text,With<TextTag>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    assets: Res<AssetServer>,
    score: Res<Scoreboard>,
    mut dead: ResMut<OnDeath>
){
    if gameover.over&& !dead.ded{
        dead.ded=true;
        commands
        .spawn(SpriteBundle{
            material: materials.add(Color::rgb(0.4, 0.4, 0.6).into()),
            transform: Transform::from_translation(Vec3::new(0.0,0.0,10.0)),
            sprite: Sprite::new(Vec2::new(800.0,800.0)),
            ..Default::default()
        })
        .spawn(TextBundle{
            text: Text{
                value: format!("You Died! Your Score:{}",score.score).to_string(),
                font: assets.load("FiraSans-Bold.ttf"),
                style:TextStyle{
                    font_size:50.0,
                    color:Color::WHITE,
                    ..Default::default()},..Default::default()
            },
            style:Style{position:Rect{left:Val::Px(400.0),right:Val::Px(200.0),top:Val::Px(200.0),bottom:Val::Px(700.0)},..Default::default()},
            transform: Transform::from_translation(Vec3::new(0.0,0.0,0.0)),
            ..Default::default()
        })
        ;
        textbnd.iter_mut().next().unwrap().value= "".to_string();
    }else{
        if !dead.ded{textbnd.iter_mut().next().unwrap().value= format!("Score: {}",score.score).to_string();} 
    }
}

fn update_size(
    commands: &mut Commands,
    mut snake: ResMut<Snake>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    score: Res<Scoreboard>,
    tail_dir: ResMut<TailDir>,
    gameover: Res<GameOver>
){
    if gameover.over{
        return;
    }
   while snake.parts.len() <score.score+3
   {
    let temp:Vec3;
       match (tail_dir.dir+2)%4 {
           0=>temp=Vec3::new(snake.speed,0.0,0.0),
           1=>temp=Vec3::new(0.0,-snake.speed,0.0),
           2=>temp=Vec3::new(-snake.speed,0.0,0.0),
           3=>temp=Vec3::new(0.0,snake.speed,0.0),
           _=>return
       }
       let tup=(temp.x+snake.parts.clone()[snake.parts.clone().len()-1].0,temp.y+snake.parts.clone()[snake.parts.clone().len()-1].1);
    commands.spawn(SpriteBundle{
        material:materials.add(Color::rgb(0.5, 0.2, 0.2).into()),
        transform: Transform::from_translation(Vec3::new(tup.0,tup.1,0.0)),
        sprite: Sprite::new(Vec2::new(32.0,32.0)),
        ..Default::default()
    }).with(Player)
    .with(Collider::Snake);
    snake.parts.push(tup);
   }
}

fn frm_time()->u64{
    let f= File::open("frame_time.txt");
    let mut f= match f{
        Ok(file)=>file,
        Err(error)=>match error.kind(){
            ErrorKind::NotFound=> match File::create("frame_time.txt"){
                Ok(fc)=> fc,
                Err(er)=>panic!("Error creating the file {:?}",er)
            },
            other_error=> {
                panic!("Error opening the file {:?}",other_error)
            }
        }
    };
    let mut result:String=String::new();
    f.read_to_string(&mut result).expect("Why");

    let result_i:u64= match result.trim().parse(){
        Ok(num)=>num,
        Err(_)=>75
    };
    result_i
}