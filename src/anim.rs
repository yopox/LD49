use bevy::prelude::*;

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


impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(update_translate_animation.system());
    }
}

pub struct TranslationAnimation {
    t0: f64,
    duration: f64,
    translation: Vec3,
    start: Vec3,
    f: easing::Functions,
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
        }
    }
}
