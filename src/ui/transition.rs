use bevy::app::{AppBuilder, EventWriter, Plugin};
use bevy::core::prelude::Time;
use bevy::ecs::entity::Entity;
use bevy::ecs::prelude::*;
use bevy::math::{Vec3, vec3};
use bevy::prelude::{DespawnRecursiveExt, Transform, Visible};
use crate::util::{Z_CARD, Z_CARD_DRAG, Z_CARD_SWITCH};

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
            .add_system(update_translate_animation.system().label("translate-animation:update"))
            .add_system(control_display_animation.system().label("display-animation:update"))
            .add_system(remove_after_animation.system().label("remove-after-animation:update"))
        ;
    }
}

pub struct DisplayBetweenAnimation {
    pub start: f64,
    pub end: f64,
}

fn control_display_animation(
    mut query: Query<(&mut Visible, &DisplayBetweenAnimation)>,
    time: Res<Time>,
) {
    for (mut visible, &DisplayBetweenAnimation { start , end }) in query.iter_mut() {
        let t = time.seconds_since_startup();
        visible.is_visible = start < t && t < end;
    }
}

pub struct RemoveAfter(pub f64);

fn remove_after_animation(
    query: Query<(Entity, &RemoveAfter)>,
    mut commands: Commands,
    time: Res<Time>,
) {
    for (e, &RemoveAfter(t)) in query.iter() {
        if time.seconds_since_startup() > t {
            commands.entity(e)
                .despawn_recursive();
        }
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

fn gen_animation(time: &Res<Time>, duration: f64, from: (f32, f32), to: (f32, f32), z_start: f32) -> TranslationAnimation {
    TranslationAnimation::from_start_end(
        time.seconds_since_startup(),
        duration,
        vec3(from.0, from.1, z_start),
        vec3(to.0, to.1, Z_CARD),
        easing::Functions::CubicOut,
    )
}

pub fn animate(time: &Res<Time>, from: (f32, f32), to: (f32, f32)) -> TranslationAnimation {
    gen_animation(time, 1.3, from, to, Z_CARD_DRAG)
}

pub fn animate_switch(time: &Res<Time>, from: (f32, f32), to: (f32, f32)) -> TranslationAnimation {
    gen_animation(time, 1., from, to, Z_CARD_SWITCH)
}

pub fn animate_fast(time: &Res<Time>, from: (f32, f32), to: (f32, f32)) -> TranslationAnimation {
    gen_animation(time, 0.5, from, to, Z_CARD_DRAG)
}