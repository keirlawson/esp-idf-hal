use esp_idf_sys::{
    esp, touch_fsm_mode_t, touch_fsm_mode_t_TOUCH_FSM_MODE_MAX, touch_fsm_mode_t_TOUCH_FSM_MODE_SW,
    touch_fsm_mode_t_TOUCH_FSM_MODE_TIMER, touch_pad_config, touch_pad_fsm_start, touch_pad_init,
    touch_pad_read_raw_data, touch_pad_set_fsm_mode, touch_pad_t, touch_pad_t_TOUCH_PAD_NUM0,
    touch_pad_t_TOUCH_PAD_NUM1, touch_pad_t_TOUCH_PAD_NUM10, touch_pad_t_TOUCH_PAD_NUM11,
    touch_pad_t_TOUCH_PAD_NUM12, touch_pad_t_TOUCH_PAD_NUM13, touch_pad_t_TOUCH_PAD_NUM14,
    touch_pad_t_TOUCH_PAD_NUM2, touch_pad_t_TOUCH_PAD_NUM3, touch_pad_t_TOUCH_PAD_NUM4,
    touch_pad_t_TOUCH_PAD_NUM5, touch_pad_t_TOUCH_PAD_NUM6, touch_pad_t_TOUCH_PAD_NUM7,
    touch_pad_t_TOUCH_PAD_NUM8, touch_pad_t_TOUCH_PAD_NUM9, EspError, CONFIG_CONSOLE_UART,
};

#[cfg(any(esp32, esp32s2, esp32s3))]
pub enum FsmMode {
    Timer,
    SW,
    Max,
}

#[cfg(any(esp32, esp32s2, esp32s3))]
impl From<FsmMode> for touch_fsm_mode_t {
    fn from(value: FsmMode) -> Self {
        match value {
            FsmMode::Timer => touch_fsm_mode_t_TOUCH_FSM_MODE_TIMER,
            FsmMode::SW => touch_fsm_mode_t_TOUCH_FSM_MODE_SW,
            FsmMode::Max => touch_fsm_mode_t_TOUCH_FSM_MODE_MAX,
        }
    }
}

#[cfg(any(esp32, esp32s2, esp32s3))]
pub enum TouchPad {
    Pad0,
    Pad1,
    Pad2,
    Pad3,
    Pad4,
    Pad5,
    Pad6,
    Pad7,
    Pad8,
    Pad9,
    Pad10,
    Pad11,
    Pad12,
    Pad13,
    Pad14,
}

#[cfg(any(esp32, esp32s2, esp32s3))]
impl From<TouchPad> for touch_pad_t {
    fn from(value: TouchPad) -> Self {
        match value {
            TouchPad::Pad0 => touch_pad_t_TOUCH_PAD_NUM0,
            TouchPad::Pad1 => touch_pad_t_TOUCH_PAD_NUM1,
            TouchPad::Pad2 => touch_pad_t_TOUCH_PAD_NUM2,
            TouchPad::Pad3 => touch_pad_t_TOUCH_PAD_NUM3,
            TouchPad::Pad4 => touch_pad_t_TOUCH_PAD_NUM4,
            TouchPad::Pad5 => touch_pad_t_TOUCH_PAD_NUM5,
            TouchPad::Pad6 => touch_pad_t_TOUCH_PAD_NUM6,
            TouchPad::Pad7 => touch_pad_t_TOUCH_PAD_NUM7,
            TouchPad::Pad8 => touch_pad_t_TOUCH_PAD_NUM8,
            TouchPad::Pad9 => touch_pad_t_TOUCH_PAD_NUM9,
            TouchPad::Pad10 => touch_pad_t_TOUCH_PAD_NUM10,
            TouchPad::Pad11 => touch_pad_t_TOUCH_PAD_NUM11,
            TouchPad::Pad12 => touch_pad_t_TOUCH_PAD_NUM12,
            TouchPad::Pad13 => touch_pad_t_TOUCH_PAD_NUM13,
            TouchPad::Pad14 => touch_pad_t_TOUCH_PAD_NUM14,
        }
    }
}

pub struct TouchConfig {
    fsm_mode: FsmMode,
    configured_pads: Vec<TouchPad>,
}

#[cfg(any(esp32, esp32s2, esp32s3))]
pub struct TouchDriver {}

#[cfg(any(esp32, esp32s2, esp32s3))]
impl TouchDriver {
    pub fn new(config: TouchConfig) -> Result<Self, EspError> {
        unsafe {
            esp!(touch_pad_init())?;
            for pad in config.configured_pads {
                esp!(touch_pad_config(pad.into()))?;
            }
            esp!(touch_pad_set_fsm_mode(config.fsm_mode.into()))?;
            esp!(touch_pad_fsm_start())?;
        }

        Ok(TouchDriver {})
    }

    pub fn read_raw_data(&mut self, pad: TouchPad) -> Result<u32, EspError> {
        let mut raw: u32 = 0;
        let result = esp!(unsafe { touch_pad_read_raw_data(pad.into(), &mut raw) });
        result.map(|_| raw)
    }
}
