use crate::util::savestate::{read_savestate_byte, LoadSavestateError, Savestate};
use crate::video::status_register::StatusMode;

pub struct PositionRegisters {
    state: DelayedState<PositionRegistersState>,
}

impl Default for PositionRegisters {
    fn default() -> Self {
        Self {
            state: DelayedState::new(PositionRegistersState::default()),
        }
    }
}

impl PositionRegisters {
    pub fn scroll(&self) -> (u8, u8) {
        self.state.state().scroll
    }

    pub fn set_scroll_x(&mut self, value: u8, mode: StatusMode) {
        let state = self.state.next_state();
        let new_state = PositionRegistersState {
            scroll: (value, state.scroll.1),
            ..*state
        };
        self.state.set_state(new_state, Self::delayed(mode));
    }

    pub fn set_scroll_y(&mut self, value: u8, mode: StatusMode) {
        let state = self.state.next_state();
        let new_state = PositionRegistersState {
            scroll: (state.scroll.0, value),
            ..*state
        };
        self.state.set_state(new_state, Self::delayed(mode));
    }

    pub fn window(&self) -> (u8, u8) {
        self.state.state().window
    }

    pub fn set_window_x(&mut self, value: u8, mode: StatusMode) {
        let state = self.state.next_state();
        let new_state = PositionRegistersState {
            window: (value, state.window.1),
            ..*state
        };
        self.state.set_state(new_state, Self::delayed(mode));
    }

    pub fn set_window_y(&mut self, value: u8, mode: StatusMode) {
        let state = self.state.next_state();
        let new_state = PositionRegistersState {
            window: (state.window.0, value),
            ..*state
        };
        self.state.set_state(new_state, Self::delayed(mode));
    }

    pub fn ly(&self) -> u8 {
        self.state.state().ly
    }

    pub fn set_ly(&mut self, value: u8, mode: StatusMode) {
        let state = self.state.next_state();
        let new_state = PositionRegistersState {
            ly: value,
            ..*state
        };
        self.state.set_state(new_state, Self::delayed(mode));
    }

    pub fn reset_ly(&mut self, mode: StatusMode) {
        let state = self.state.next_state();
        let new_state = PositionRegistersState { ly: 0, ..*state };
        self.state.set_state(new_state, Self::delayed(mode));
    }

    pub fn lyc(&self) -> u8 {
        self.state.state().lyc
    }

    pub fn set_lyc(&mut self, value: u8, mode: StatusMode) {
        let state = self.state.next_state();
        let new_state = PositionRegistersState {
            lyc: value,
            ..*state
        };
        self.state.set_state(new_state, Self::delayed(mode));
    }

    fn delayed(mode: StatusMode) -> bool {
        mode == StatusMode::LCDTransfer
    }

    pub fn on_mode_change(&mut self, mode: StatusMode) {
        if !Self::delayed(mode) {
            self.state.apply_next_state();
        }
    }
}

impl Savestate for PositionRegisters {
    fn dump_savestate(&self, buffer: &mut Vec<u8>) {
        self.state.next_state.dump_savestate(buffer);
    }

    fn load_savestate<'a>(
        &mut self,
        buffer: &mut std::slice::Iter<'a, u8>,
    ) -> Result<(), LoadSavestateError> {
        self.state.next_state.load_savestate(buffer)?;
        self.state.apply_next_state();
        Ok(())
    }
}

#[derive(Default, Copy, Clone, Debug)]
struct PositionRegistersState {
    pub scroll: (u8, u8),
    pub window: (u8, u8),
    pub ly: u8,
    pub lyc: u8,
}

impl Savestate for PositionRegistersState {
    fn dump_savestate(&self, buffer: &mut Vec<u8>) {
        buffer.push(self.scroll.0);
        buffer.push(self.scroll.1);
        buffer.push(self.window.0);
        buffer.push(self.window.1);
        buffer.push(self.ly);
        buffer.push(self.lyc);
    }

    fn load_savestate<'a>(
        &mut self,
        buffer: &mut std::slice::Iter<'a, u8>,
    ) -> Result<(), LoadSavestateError> {
        self.scroll.0 = read_savestate_byte(buffer)?;
        self.scroll.1 = read_savestate_byte(buffer)?;
        self.window.0 = read_savestate_byte(buffer)?;
        self.window.1 = read_savestate_byte(buffer)?;
        self.ly = read_savestate_byte(buffer)?;
        self.lyc = read_savestate_byte(buffer)?;
        Ok(())
    }
}

struct DelayedState<T: Copy + Clone> {
    state: T,
    next_state: T,
}

impl<T: Copy + Clone> DelayedState<T> {
    pub fn new(state: T) -> DelayedState<T> {
        DelayedState {
            state,
            next_state: state,
        }
    }

    pub fn state(&self) -> &T {
        &self.state
    }

    pub fn next_state(&self) -> &T {
        &self.next_state
    }

    pub fn set_state(&mut self, new_state: T, delayed: bool) {
        self.next_state = new_state;
        if !delayed {
            self.apply_next_state();
        }
    }

    pub fn apply_next_state(&mut self) {
        self.state = self.next_state;
    }
}
