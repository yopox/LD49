use bevy::math::{vec2, vec3, Vec4Swizzles};
use bevy::prelude::*;

use crate::MainCamera;
use crate::util::{cursor_pos, overlap};

pub struct AnimationPlugin;

pub mod easing {
    pub enum Functions {
        QuadIn,
        QuadOut,
        QuadInOut,
        CubicIn,
        CubicOut,
        CubicInOut,
        QuartIn,
        QuartOut,
        QuartInOut,
        QuintIn,
        QuintOut,
        QuintInOut,
        SineIn,
        SineOut,
        SineInOut,
        CircIn,
        CircOut,
        CircInOut,
        ExpoIn,
        ExpoOut,
        ExpoInOut,
        ElasticIn,
        ElasticOut,
        ElasticInOut,
        BackIn,
        BackOut,
        BackInOut,
        BounceIn,
        BounceOut,
        BounceInOut,
    }

    pub fn apply(f: &Functions, t: f32) -> f32 {
        (match f {
            Functions::QuadIn => { ezing::quad_in }
            Functions::QuadOut => { ezing::quad_out }
            Functions::QuadInOut => { ezing::quad_inout }
            Functions::CubicIn => { ezing::cubic_in }
            Functions::CubicOut => { ezing::cubic_out }
            Functions::CubicInOut => { ezing::cubic_inout }
            Functions::QuartIn => { ezing::quad_in }
            Functions::QuartOut => { ezing::quart_out }
            Functions::QuartInOut => { ezing::quart_inout }
            Functions::QuintIn => { ezing::quint_in }
            Functions::QuintOut => { ezing::quint_out }
            Functions::QuintInOut => { ezing::quint_inout }
            Functions::SineIn => { ezing::sine_in }
            Functions::SineOut => { ezing::sine_out }
            Functions::SineInOut => { ezing::sine_inout }
            Functions::CircIn => { ezing::circ_in }
            Functions::CircOut => { ezing::circ_out }
            Functions::CircInOut => { ezing::circ_inout }
            Functions::ExpoIn => { ezing::expo_in }
            Functions::ExpoOut => { ezing::expo_out }
            Functions::ExpoInOut => { ezing::expo_inout }
            Functions::ElasticIn => { ezing::elastic_in }
            Functions::ElasticOut => { ezing::elastic_out }
            Functions::ElasticInOut => { ezing::elastic_inout }
            Functions::BackIn => { ezing::back_in }
            Functions::BackOut => { ezing::back_out }
            Functions::BackInOut => { ezing::back_inout }
            Functions::BounceIn => { ezing::bounce_in }
            Functions::BounceOut => { ezing::bounce_out }
            Functions::BounceInOut => { ezing::bounce_inout }
        })(t)
    }
}

pub struct TransitionOver(pub Entity);

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_event::<TransitionOver>()
            .add_system(update_translate_animation.system().label("translate-animation:update"));
    }
}

pub struct TranslationAnimation {
    pub t0: f64,
    pub duration: f64,
    pub translation: Vec3,
    pub start: Vec3,
    pub f: easing::Functions,
}

impl TranslationAnimation {
    pub fn from_start_end(t0: f64, duration: f64, start: Vec3, end: Vec3, f: easing::Functions) -> TranslationAnimation {
        TranslationAnimation {
            t0,
            duration,
            start,
            f,
            translation: end - start,
        }
    }
}


fn update_translate_animation(
    time: Res<Time>,
    mut ev_transition: EventWriter<TransitionOver>,
    mut query: Query<(Entity, &TranslationAnimation, &mut Transform)>,
    mut commands: Commands,
) {
    let t1 = time.seconds_since_startup();
    for (e, TranslationAnimation { duration, t0, start, translation: speed, f }, mut transform) in query.iter_mut() {
        let t = t1 - t0;
        if t < *duration {
            let t = (t / duration) as f32;
            transform.translation = *start + *speed * easing::apply(f, t);
        } else {
            commands.entity(e).remove::<TranslationAnimation>();
            ev_transition.send(TransitionOver(e));
        }
    }
}

pub struct DragAndDropPlugin;

pub struct Draggable {
    pub size: Vec2,
}

pub struct Dragged;
pub struct Dropped(pub Entity);
pub const DROP_BORDER: f32 = 10.;

impl Plugin for DragAndDropPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_event::<Dropped>()
            .add_system(drag_update.system().label("drag:update"))
            .add_system(drop_update.system().label("drag:end").after("drag:update"))
            .add_system(begin_drag.system().label("drag:begin"))
        ;
    }
}

fn drag_update(
    windows: Res<Windows>,
    mut queries: QuerySet<(
        Query<&Transform, With<MainCamera>>,
        Query<&mut Transform, With<Dragged>>,
    )>,
) {
    let window = windows.get_primary().unwrap();
    if let Some(cursor) = cursor_pos(window, queries.q0().single().unwrap()) {
        // Get hovered card id & transform
        for mut transform in queries.q1_mut().iter_mut() {
            transform.translation.x = cursor.x;
            transform.translation.y = cursor.y;
        }
    }
}

fn drop_update(
    mut commands: Commands,
    mut ev_dropped: EventWriter<Dropped>,
    btn: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    queries: QuerySet<(
        Query<&Transform, With<MainCamera>>,
        Query<Entity, With<Dragged>>,
    )>,
) {
    if btn.just_released(MouseButton::Left) {
        let window = windows.get_primary().unwrap();
        // Get hovered card id & transform
        if let Some(cursor) = cursor_pos(window, queries.q0().single().unwrap()) {
            for e in queries.q1().iter() {
                commands.entity(e)
                    .remove::<Dragged>();
                ev_dropped.send(Dropped(e));
            }
        }
    }
}

fn begin_drag(
    mut commands: Commands,
    btn: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    mut queries: QuerySet<(
        Query<&Transform, With<MainCamera>>,
        Query<(Entity, &Draggable, &mut Transform), Without<TranslationAnimation>>,
    )>,
) {
    if btn.just_pressed(MouseButton::Left) {
        // Start dragging a card
        let window = windows.get_primary().unwrap();
        if let Some(cursor) = cursor_pos(window, queries.q0().single().unwrap()) {
            // Get hovered card id & transform
            for (e, draggable, mut transform) in queries.q1_mut().iter_mut() {
                if overlap(cursor.xyz(), transform.translation, (draggable.size.x, draggable.size.y)) {
                    commands.entity(e).insert(Dragged);
                    transform.translation.x = cursor.x;
                    transform.translation.y = cursor.y;
                    break;
                }
            }
        }
    }
}

fn gen_animation(time: &Res<Time>, duration: f64, from: (f32, f32), to: (f32, f32)) -> TranslationAnimation {
    TranslationAnimation::from_start_end(
        time.seconds_since_startup(),
        duration,
        vec3(from.0, from.1, 0.),
        vec3(to.0, to.1, 0.),
        easing::Functions::CubicOut,
    )
}

pub fn animate(time: &Res<Time>, from: (f32, f32), to: (f32, f32)) -> TranslationAnimation {
    gen_animation(time, 1.3, from, to)
}

pub fn animate_switch(time: &Res<Time>, from: (f32, f32), to: (f32, f32)) -> TranslationAnimation {
    gen_animation(time, 1., from, to)
}

pub fn animate_fast(time: &Res<Time>, from: (f32, f32), to: (f32, f32)) -> TranslationAnimation {
    gen_animation(time, 0.5, from, to)
}