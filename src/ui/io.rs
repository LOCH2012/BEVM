use crate::ui::gui::GuiState;
use crate::ui::window::Tool;
use imgui::sys::{
    igBeginTable, igEndTable, igTableNextColumn, igTableNextRow, ImGuiTableFlags_None,
    ImGuiTableRowFlags_None, ImVec2,
};

use imgui::{ImString, Io, Ui};
use std::os::raw::c_int;

pub struct IOTool;

impl IOTool {
    pub fn new() -> IOTool {
        IOTool {}
    }
}

impl Tool for IOTool {
    fn draw(&mut self, ui: &Ui, _io: &Io, state: &mut GuiState) {
        let w_tok = ui.push_item_width(150.0);

        unsafe {
            igBeginTable(
                ImString::new("io_devices").as_ptr(),
                3,
                ImGuiTableFlags_None as c_int,
                ImVec2::zero(),
                0.0,
            );
        }

        for (id, cell) in &mut state.computer.io_devices.iter_mut().enumerate() {
            unsafe {
                igTableNextRow(ImGuiTableRowFlags_None as c_int, 0.0);
                igTableNextColumn();
            }
            let id_tok = ui.push_id_int(id as i32);

            ui.text(format!("ВУ-{}:", id));
            unsafe { igTableNextColumn() };

            let mut input = String::with_capacity(2);
            input.push_str(format!("{:0>2X}", cell.data).as_str());
            if ui
                .input_text("", &mut input)
                .chars_hexadecimal(true)
                .build()
            {
                if let Ok(parsed) = u8::from_str_radix(&input, 16) {
                    cell.data = parsed
                }
            }
            unsafe { igTableNextColumn() };

            ui.checkbox("Готов", &mut cell.ready);

            id_tok.pop();
        }

        unsafe {
            igEndTable();
        }

        w_tok.end();

        if state.computer.io_devices.iter().any(|a| a.ready) {
            if state.computer.registers.get_allow_interupt() {
                state.computer.registers.set_interrupt(true);
            }
        } else {
            state.computer.registers.set_interrupt(false);
        }
    }
}
