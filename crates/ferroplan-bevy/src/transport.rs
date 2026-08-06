//! The transport deck, docked bottom-screen: a run/hold switch, a scrubbable
//! timeline — click or drag to seek — notched once per step, a molten fill and
//! playhead tracking the burn, and a step/time readout. It shadows the keyboard
//! rig (Space / ←→ / R) so the whole run stays workable off the mouse alone.
//!
//! The deck only lights up while a plan with steps is loaded. The track hands
//! back the pointer's normalized position via `RelativeCursorPosition`, mapped
//! straight onto timeline `t` on press or drag.

use bevy::prelude::*;
use bevy::ui::RelativeCursorPosition;

use crate::anim::Plan;

/// Lit while the pointer sits over the deck — tells world interaction (node
/// selection) to look away from clicks that are really just scrubbing.
#[derive(Resource, Default)]
pub struct Transport {
    pub hovering: bool,
    /// The step count the notches were last cut for — rebuild only when it moves.
    built_for: usize,
}

#[derive(Component)]
pub struct TransportBar;
#[derive(Component)]
pub struct PlayButton;
#[derive(Component)]
pub struct PlayIcon;
#[derive(Component)]
pub struct ScrubTrack;
#[derive(Component)]
pub struct ScrubFill;
#[derive(Component)]
pub struct Playhead;
#[derive(Component)]
pub struct StepNotch;
#[derive(Component)]
pub struct TransportLabel;

/// One notch burns per step while the run's still short enough to read; past
/// that, the fill and playhead alone carry the signal.
const MAX_NOTCHES: usize = 80;

pub fn setup_transport(mut commands: Commands) {
    commands
        .spawn((
            TransportBar,
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(0.0),
                right: Val::Px(340.0), // clear the inspector panel
                bottom: Val::Px(0.0),
                height: Val::Px(54.0),
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                column_gap: Val::Px(12.0),
                padding: UiRect::axes(Val::Px(16.0), Val::Px(10.0)),
                border: UiRect::top(Val::Px(1.0)),
                ..default()
            },
            BackgroundColor(crate::palette::PANEL_BLUR),
            BorderColor::all(crate::palette::EDGE2),
            Visibility::Hidden,
        ))
        .with_children(|p| {
            // play / pause button
            p.spawn((
                PlayButton,
                Button,
                Node {
                    width: Val::Px(34.0),
                    height: Val::Px(28.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    border_radius: BorderRadius::all(Val::Px(6.0)),
                    ..default()
                },
                BackgroundColor(crate::palette::PANEL2),
            ))
            .with_children(|b| {
                b.spawn((
                    PlayIcon,
                    Text::new("\u{25B6}"), // ▶
                    TextFont {
                        font_size: 14.0_f32.into(),
                        ..default()
                    },
                    TextColor(crate::palette::INK),
                ));
            });

            // scrub track (grows to fill); fill, notches and playhead are absolute
            // children positioned by percentage of `t / n`.
            p.spawn((
                ScrubTrack,
                Button, // so it reports Interaction
                RelativeCursorPosition::default(),
                Node {
                    flex_grow: 1.0,
                    height: Val::Px(8.0),
                    border_radius: BorderRadius::all(Val::Px(4.0)),
                    overflow: Overflow::clip(),
                    ..default()
                },
                BackgroundColor(crate::palette::EDGE),
            ))
            .with_children(|t| {
                t.spawn((
                    ScrubFill,
                    Node {
                        position_type: PositionType::Absolute,
                        left: Val::Px(0.0),
                        top: Val::Px(0.0),
                        bottom: Val::Px(0.0),
                        width: Val::Percent(0.0),
                        ..default()
                    },
                    BackgroundColor(crate::palette::ACC),
                ));
                t.spawn((
                    Playhead,
                    Node {
                        position_type: PositionType::Absolute,
                        top: Val::Px(0.0),
                        bottom: Val::Px(0.0),
                        left: Val::Percent(0.0),
                        width: Val::Px(3.0),
                        ..default()
                    },
                    BackgroundColor(crate::palette::INK),
                ));
            });

            // step / time readout
            p.spawn((
                TransportLabel,
                Text::new(""),
                TextFont {
                    font_size: 12.0_f32.into(),
                    ..default()
                },
                TextColor(crate::palette::MUT),
                Node {
                    min_width: Val::Px(150.0),
                    ..default()
                },
            ));
        });
}

/// Keep the deck dark until a plan with steps actually loads.
pub fn transport_visibility(plan: Res<Plan>, mut bar: Query<&mut Visibility, With<TransportBar>>) {
    let Ok(mut vis) = bar.single_mut() else {
        return;
    };
    let want = if plan.steps.is_empty() {
        Visibility::Hidden
    } else {
        Visibility::Visible
    };
    if *vis != want {
        *vis = want;
    }
}

/// Recut the per-step notches the instant the run's length shifts underfoot.
pub fn rebuild_notches(
    mut commands: Commands,
    plan: Res<Plan>,
    mut state: ResMut<Transport>,
    track: Query<Entity, With<ScrubTrack>>,
    notches: Query<Entity, With<StepNotch>>,
) {
    let n = plan.steps.len();
    if n == state.built_for {
        return;
    }
    state.built_for = n;
    for e in &notches {
        commands.entity(e).despawn();
    }
    let Ok(track) = track.single() else {
        return;
    };
    if !(2..=MAX_NOTCHES).contains(&n) {
        return;
    }
    // One notch per action start — evenly spaced for classic plans, placed by
    // start time for temporal plans (so the spacing mirrors the Gantt).
    let fracs: Vec<f32> = plan
        .steps
        .iter()
        .enumerate()
        .map(|(i, s)| plan.start_frac(s, i))
        .filter(|&f| f > 0.001)
        .collect();
    commands.entity(track).with_children(|t| {
        for f in fracs {
            t.spawn((
                StepNotch,
                Node {
                    position_type: PositionType::Absolute,
                    top: Val::Px(0.0),
                    bottom: Val::Px(0.0),
                    left: Val::Percent(100.0 * f),
                    width: Val::Px(1.0),
                    ..default()
                },
                BackgroundColor(crate::palette::BG2),
            ));
        }
    });
}

/// Slave the fill width, playhead, play icon, and readout to whatever `Plan` says.
pub fn transport_sync(
    plan: Res<Plan>,
    mut fill: Query<&mut Node, (With<ScrubFill>, Without<Playhead>)>,
    mut head: Query<&mut Node, (With<Playhead>, Without<ScrubFill>)>,
    mut icon: Query<&mut Text, (With<PlayIcon>, Without<TransportLabel>)>,
    mut label: Query<&mut Text, (With<TransportLabel>, Without<PlayIcon>)>,
) {
    let frac = plan.frac() * 100.0;
    if let Ok(mut f) = fill.single_mut() {
        f.width = Val::Percent(frac);
    }
    if let Ok(mut h) = head.single_mut() {
        h.left = Val::Percent(frac);
    }
    if let Ok(mut t) = icon.single_mut() {
        let glyph = if plan.playing { "\u{23F8}" } else { "\u{25B6}" }; // ⏸ / ▶
        if t.0 != glyph {
            *t = Text::new(glyph);
        }
    }
    if let Ok(mut l) = label.single_mut() {
        *l = Text::new(readout(&plan));
    }
}

/// Temporal reads `t=…/makespan · k active · <action>` — whatever's still
/// burning at this instant. Classic reads flat: `step k/n · <action>`.
fn readout(plan: &Plan) -> String {
    let n = plan.steps.len();
    if n == 0 {
        return String::new();
    }
    if plan.temporal {
        let now = plan.t;
        let active: Vec<&str> = plan
            .steps
            .iter()
            .filter(|s| {
                let st = s.time.unwrap_or(0.0) as f32;
                let en = st + s.duration.unwrap_or(0.0) as f32;
                now + 1e-3 >= st && now <= en + 1e-3
            })
            .map(|s| s.action.as_str())
            .collect();
        let lead = active.first().copied().unwrap_or("—").to_lowercase();
        return format!(
            "t={:.2}/{:.2}  ·  {} active  ·  {}",
            now,
            plan.span(),
            active.len(),
            lead
        );
    }
    let k = (plan.t.floor() as usize).min(n - 1);
    if plan.t as usize >= n {
        return format!("done · {n} steps");
    }
    let step = &plan.steps[k];
    format!("step {}/{} · {}", k + 1, n, step.action.to_lowercase())
}

/// Field the play button and any scrub across the track.
pub fn transport_input(
    mouse: Res<ButtonInput<MouseButton>>,
    mut transport: ResMut<Transport>,
    mut plan: ResMut<Plan>,
    play_btn: Query<&Interaction, (With<PlayButton>, Changed<Interaction>)>,
    track: Query<(&Interaction, &RelativeCursorPosition), With<ScrubTrack>>,
) {
    if plan.steps.is_empty() {
        transport.hovering = false;
        return;
    }
    let span = plan.span();

    // play / pause toggle
    for it in &play_btn {
        if *it == Interaction::Pressed {
            if plan.t >= span {
                plan.t = 0.0;
            }
            plan.playing = !plan.playing;
        }
    }

    // scrub: while the pointer is over the track and the button is held, map the
    // normalized x to a timeline position and pause.
    let mut hovering = false;
    if let Ok((_, rel)) = track.single() {
        hovering = rel.cursor_over();
        if hovering && mouse.pressed(MouseButton::Left) {
            if let Some(p) = rel.normalized {
                plan.t = (p.x.clamp(0.0, 1.0) * span).min(span);
                plan.playing = false;
            }
        }
    }
    transport.hovering = hovering;
}
