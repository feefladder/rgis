#![warn(
    clippy::unwrap_used,
    clippy::cast_lossless,
    clippy::unimplemented,
    clippy::indexing_slicing,
    clippy::expect_used
)]

use bevy::{ecs::system::SystemParam, prelude::*};
use bevy_egui::egui;
use std::{collections, marker};

mod add_layer_window;
mod bottom_panel;
mod change_crs_window;
mod debug_window;
mod events;
mod feature_properties_window;
mod manage_layer_window;
mod message_window;
mod operation_window;
mod side_panel;
mod systems;
mod top_panel;
mod widgets;

trait Window: egui::Widget + SystemParam {
    type Item<'world, 'state>: Window<State = Self::State>;

    fn is_visible(&self) -> bool;
    fn set_visible(&mut self, visible: bool);
    fn egui_ctx(&mut self) -> Mut<bevy_egui::EguiContext>;
    fn render(&mut self) {
        if self.is_visible() {
            egui::Window::new("Window").show(self.egui_ctx().get_mut(), |ui| {
                ui.label("FOO");
            });
        }
    }
}
pub struct Plugin;

#[derive(Copy, Clone, Resource)]
pub struct SidePanelWidth(pub f32);

#[derive(Copy, Clone, Resource)]
pub struct TopPanelHeight(pub f32);

#[derive(Copy, Clone, Resource)]
pub struct BottomPanelHeight(pub f32);

#[derive(bevy::ecs::system::SystemParam, Resource)]
pub struct UiMargins<'w, 's> {
    pub left: Res<'w, SidePanelWidth>,
    pub top: Res<'w, TopPanelHeight>,
    pub bottom: Res<'w, BottomPanelHeight>,
    #[system_param(ignore)]
    marker: marker::PhantomData<&'s usize>,
}

impl<'w, 's> UiMargins<'w, 's> {
    // pub fn to_ui_rect(&self) -> bevy::ui::UiRect {
    //     bevy::ui::UiRect {
    //         left: Val::Px(self.left.0),
    //         top: Val::Px(self.top.0),
    //         bottom: Val::Px(self.bottom.0),
    //         right: Val::Px(0.),
    //     }
    // }
}

#[derive(Default, Resource)]
pub struct MessageWindowState {
    is_visible: bool,
    message: Option<String>,
}

#[derive(Default, Resource)]
pub struct ManageLayerWindowState {
    layer_id: Option<rgis_layer_id::LayerId>,
    is_visible: bool,
}

#[derive(Default)]
pub struct FeaturePropertiesWindowState {
    properties: Option<geo_features::Properties>,
    is_visible: bool,
}

#[derive(Default)]
struct OperationWindowState {
    is_visible: bool,
    operation: Option<Box<dyn Send + Sync + rgis_geo_ops::Operation>>,
    feature_collection: geo_projected::Unprojected<geo_features::FeatureCollection>,
}

const DEBUG_STATS_HISTORY_LEN: usize = 100;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(bevy_egui::EguiPlugin)
            .insert_resource(add_layer_window::SelectedFile(None))
            .insert_resource(TopPanelHeight(0.))
            .insert_resource(BottomPanelHeight(0.))
            .insert_resource(SidePanelWidth(0.))
            // TODO: remove the below resource and replace with Local state
            .insert_resource(debug_window::DebugStatsWindowState {
                timer: Timer::from_seconds(0.3, TimerMode::Repeating),
                is_visible: false,
                history: collections::VecDeque::with_capacity(DEBUG_STATS_HISTORY_LEN),
            })
            .add_event::<events::OpenOperationWindowEvent>();

        systems::configure(app);
    }
}
