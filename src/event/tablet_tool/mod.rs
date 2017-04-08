use ::ffi;
use ::{FromRaw, AsRaw, ButtonState};
use super::EventTrait;

mod tool;
pub use self::tool::TabletTool;

pub trait TabletToolEventTrait: AsRaw<ffi::libinput_event_tablet_tool>
{
    ffi_func!(fn time, ffi::libinput_event_tablet_tool_get_time, u32);
    ffi_func!(fn time_usec, ffi::libinput_event_tablet_tool_get_time_usec, u64);
    ffi_func!(fn distance_has_changed, ffi::libinput_event_tablet_tool_distance_has_changed, bool);
    ffi_func!(fn distance, ffi::libinput_event_tablet_tool_get_distance, f64);
    ffi_func!(fn dx, ffi::libinput_event_tablet_tool_get_dx, f64);
    ffi_func!(fn dy, ffi::libinput_event_tablet_tool_get_dy, f64);
    ffi_func!(fn pressure_has_changed, ffi::libinput_event_tablet_tool_pressure_has_changed, bool);
    ffi_func!(fn pressure, ffi::libinput_event_tablet_tool_get_pressure, f64);
    ffi_func!(fn rotation_has_changed, ffi::libinput_event_tablet_tool_rotation_has_changed, bool);
    ffi_func!(fn rotation, ffi::libinput_event_tablet_tool_get_rotation, f64);
    ffi_func!(fn slider_has_changed, ffi::libinput_event_tablet_tool_slider_has_changed, bool);
    ffi_func!(fn slider_position, ffi::libinput_event_tablet_tool_get_slider_position, f64);
    ffi_func!(fn tilt_x_has_changed, ffi::libinput_event_tablet_tool_tilt_x_has_changed, bool);
    ffi_func!(fn tilt_y_has_changed, ffi::libinput_event_tablet_tool_tilt_y_has_changed, bool);
    ffi_func!(fn tilt_x, ffi::libinput_event_tablet_tool_get_tilt_x, f64);
    ffi_func!(fn tilt_y, ffi::libinput_event_tablet_tool_get_tilt_y, f64);
    ffi_func!(fn wheel_has_changed, ffi::libinput_event_tablet_tool_wheel_has_changed, bool);
    ffi_func!(fn wheel_delta, ffi::libinput_event_tablet_tool_get_wheel_delta, f64);
    ffi_func!(fn wheel_delta_discrete, ffi::libinput_event_tablet_tool_get_wheel_delta_discrete, f64);
    ffi_func!(fn x_has_changed, ffi::libinput_event_tablet_tool_x_has_changed, bool);
    ffi_func!(fn y_has_changed, ffi::libinput_event_tablet_tool_y_has_changed, bool);
    ffi_func!(fn x, ffi::libinput_event_tablet_tool_get_x, f64);
    ffi_func!(fn y, ffi::libinput_event_tablet_tool_get_y, f64);

    fn x_transformed(&self, width: u32) -> f64 {
        unsafe { ffi::libinput_event_tablet_tool_get_x_transformed(self.as_raw_mut(), width) }
    }

    fn y_transformed(&self, height: u32) -> f64 {
        unsafe { ffi::libinput_event_tablet_tool_get_x_transformed(self.as_raw_mut(), height) }
    }

    fn tool(&self) -> TabletTool {
        unsafe { TabletTool::from_raw(ffi::libinput_event_tablet_tool_get_tool(self.as_raw_mut())) }
    }

    fn into_tablet_tool_event(self) -> TabletToolEvent where Self: Sized {
        unsafe { TabletToolEvent::from_raw(self.as_raw_mut()) }
    }
}

impl<T: AsRaw<ffi::libinput_event_tablet_tool>> TabletToolEventTrait for T {}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TabletToolEvent {
    Axis(TabletToolAxisEvent),
    Proximity(TabletToolProximityEvent),
    Tip(TabletToolTipEvent),
    Button(TabletToolButtonEvent),
}

impl EventTrait for TabletToolEvent {
    fn as_raw_event(&self) -> *mut ffi::libinput_event {
        match *self {
            TabletToolEvent::Axis(ref event) => event.as_raw_event(),
            TabletToolEvent::Proximity(ref event) => event.as_raw_event(),
            TabletToolEvent::Tip(ref event) => event.as_raw_event(),
            TabletToolEvent::Button(ref event) => event.as_raw_event(),
        }
    }
}

impl FromRaw<ffi::libinput_event_tablet_tool> for TabletToolEvent {
    unsafe fn from_raw(event: *mut ffi::libinput_event_tablet_tool) -> Self {
        let base = ffi::libinput_event_tablet_tool_get_base_event(event);
        match ffi::libinput_event_get_type(base) {
            ffi::libinput_event_type::LIBINPUT_EVENT_TABLET_TOOL_AXIS =>
                TabletToolEvent::Axis(TabletToolAxisEvent::from_raw(event)),
            ffi::libinput_event_type::LIBINPUT_EVENT_TABLET_TOOL_PROXIMITY =>
                TabletToolEvent::Proximity(TabletToolProximityEvent::from_raw(event)),
            ffi::libinput_event_type::LIBINPUT_EVENT_TABLET_TOOL_TIP =>
                TabletToolEvent::Tip(TabletToolTipEvent::from_raw(event)),
            ffi::libinput_event_type::LIBINPUT_EVENT_TABLET_TOOL_BUTTON =>
                TabletToolEvent::Button(TabletToolButtonEvent::from_raw(event)),
            _ => unreachable!(),
        }
    }
}

impl AsRaw<ffi::libinput_event_tablet_tool> for TabletToolEvent {
    fn as_raw(&self) -> *const ffi::libinput_event_tablet_tool {
        match *self {
            TabletToolEvent::Axis(ref event) => event.as_raw(),
            TabletToolEvent::Proximity(ref event) => event.as_raw(),
            TabletToolEvent::Tip(ref event) => event.as_raw(),
            TabletToolEvent::Button(ref event) => event.as_raw(),
        }
    }
}

ffi_event_struct!(struct TabletToolAxisEvent, ffi::libinput_event_tablet_tool, ffi::libinput_event_tablet_tool_get_base_event);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProximityState {
    Out,
    In,
}

ffi_event_struct!(struct TabletToolProximityEvent, ffi::libinput_event_tablet_tool, ffi::libinput_event_tablet_tool_get_base_event);

impl TabletToolProximityEvent {
    pub fn proximity_state(&self) -> ProximityState {
        match unsafe { ffi::libinput_event_tablet_tool_get_proximity_state(self.as_raw_mut()) } {
            ffi::libinput_tablet_tool_proximity_state::LIBINPUT_TABLET_TOOL_PROXIMITY_STATE_OUT => ProximityState::Out,
            ffi::libinput_tablet_tool_proximity_state::LIBINPUT_TABLET_TOOL_PROXIMITY_STATE_IN => ProximityState::In,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TipState {
    Up,
    Down,
}

ffi_event_struct!(struct TabletToolTipEvent, ffi::libinput_event_tablet_tool, ffi::libinput_event_tablet_tool_get_base_event);

impl TabletToolTipEvent {
    pub fn tip_state(&self) -> TipState {
        match unsafe { ffi::libinput_event_tablet_tool_get_tip_state(self.as_raw_mut()) } {
            ffi::libinput_tablet_tool_tip_state::LIBINPUT_TABLET_TOOL_TIP_UP => TipState::Up,
            ffi::libinput_tablet_tool_tip_state::LIBINPUT_TABLET_TOOL_TIP_DOWN => TipState::Down,
        }
    }
}

ffi_event_struct!(struct TabletToolButtonEvent, ffi::libinput_event_tablet_tool, ffi::libinput_event_tablet_tool_get_base_event);

impl TabletToolButtonEvent {
    ffi_func!(pub fn button, ffi::libinput_event_tablet_tool_get_button, u32);
    ffi_func!(pub fn seat_button_count, ffi::libinput_event_tablet_tool_get_seat_button_count, u32);

    pub fn button_state(&self) -> ButtonState {
        match unsafe { ffi::libinput_event_tablet_tool_get_button_state(self.as_raw_mut()) } {
            ffi::libinput_button_state::LIBINPUT_BUTTON_STATE_PRESSED => ButtonState::Pressed,
            ffi::libinput_button_state::LIBINPUT_BUTTON_STATE_RELEASED => ButtonState::Released,
        }
    }
}