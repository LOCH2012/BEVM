use crate::parse::{CommandInfo, Parser};

use core::ops::{BitAnd, BitOr, BitXor};
use imgui::Ui;
use std::collections::HashMap;
use std::rc::Rc;

pub struct GeneralParser {
    sorted: Vec<Rc<dyn GeneralCommand>>,
    mnemonic_map: HashMap<String, Rc<dyn GeneralCommand>>,
}

impl GeneralParser {
    fn register<T>(&mut self, command: T)
    where
        T: 'static,
        T: GeneralCommand,
    {
        let rc1 = Rc::new(command);
        let rc2 = rc1.clone();
        self.sorted.push(rc1);
        self.mnemonic_map.insert(rc2.mnemonic().to_string(), rc2);
    }

    pub fn new() -> GeneralParser {
        let mut parser = GeneralParser {
            sorted: vec![],
            mnemonic_map: HashMap::new(),
        };

        parser.register(SimpleCommand::new(
            0xFF00,
            "HZF",
            "Команда для которой не задано поведение. То есть она ничего не делает.",
        ));
        parser.register(SimpleCommand::new(
            0xFE00,
            "HZE",
            "Команда для которой не задано поведение. То есть она ничего не делает.",
        ));
        parser.register(SimpleCommand::new(
            0xFD00,
            "HZD",
            "Команда для которой не задано поведение. То есть она ничего не делает.",
        ));
        parser.register(SimpleCommand::new(
            0xFC00,
            "HZC",
            "Команда для которой не задано поведение. То есть она ничего не делает.",
        ));

        parser.register(SimpleCommand::new(0xF700, "ROR", "Сдвигает биты в регистре А вправо. При этом содержимое С попадает в старший бит А, а младший бит А попадает в С"));
        parser.register(SimpleCommand::new(0xFB00, "DI", "Запрещает прерывания"));

        parser.register(SimpleCommand::new(0xF300, "CLC", "Устанавливает С в 0"));
        parser.register(SimpleCommand::new(
            0xF500,
            "CMC",
            "Инвертирует С. То есть, если С было равно 1, оно станет 0 и наоборот.",
        ));
        parser.register(SimpleCommand::new(0xF600, "ROL", "Сдвигает биты в регистре А влево. При этом содержимое С попадает в младший бит А, а старший бит А попадает в С."));
        parser.register(SimpleCommand::new(
            0xF900,
            "DEC",
            "Уменьшает значение А на 1",
        ));
        parser.register(SimpleCommand::new(0xFA00, "EI", "Разрешает прерывания"));
        parser.register(SimpleCommand::new(
            0xF200,
            "CLA",
            "Устанавливает значение регистра А в 0",
        ));
        parser.register(SimpleCommand::new(0xF400, "CMA", "Инвертирует содержимое регистра А. То есть каждый бит регистра А, который равен 0, станет 1 и наоборот."));
        parser.register(SimpleCommand::new(
            0xF800,
            "INC",
            "Увеличивает значение регистра А на 1",
        ));
        parser.register(SimpleCommand::new(0xF100, "NOP", "Команда, которая не делает ничего. Удобно применять вместе с ISZ для инкремента какой-либо ячейки памяти."));
        parser.register(AddressCommand::new_io(
            0xE300,
            "OUT",
            "Присваивает указаному ВУ значение из регистра А",
        ));
        parser.register(SimpleCommand::new(0xF000, "HLT", "Выключает ЭВМ."));
        parser.register(AddressCommand::new_io(0xE100, "TSF", "Присваивает 6 биту регистра РС статус готовности указанного ВУ. Затем, если 6 бит РС равен единице, регистр СК увеличивается на единицу."));
        parser.register(AddressCommand::new_io(
            0xE200,
            "IN",
            "Берет значение из данного ВУ и кладет его в 8 младших бит регистра А",
        ));
        parser.register(AddressCommand::new_address(
            0xB000,
            "BEQ",
            "Присваивает регистру СК значение X, если регистр А равен 0.",
        ));
        parser.register(AddressCommand::new_io(
            0xE000,
            "CLF",
            "Устанавливает флаг готовности данного ВУ в 0.",
        ));
        parser.register(AddressCommand::new_address(
            0x3000,
            "MOV",
            "Присваивает ячейке по адресу X значение из регистра А",
        ));
        parser.register(AddressCommand::new_address(0x5000, "ADC", "Складывает значение из ячейки по адресу X с регистром А и добавляет 1, если С равен 1."));
        parser.register(AddressCommand::new_address(
            0x6000,
            "SUB",
            "Вычитает значение ячейки по адресу X из регистра А.",
        ));
        parser.register(AddressCommand::new_address(
            0x9000,
            "BPL",
            "Присваивает регистру СК значение X, если значение в регистре А больше или равно 0.",
        ));
        parser.register(AddressCommand::new_address(
            0xA000,
            "BMI",
            "Присваивает регистру СК значение X, если значение в регистре А строго меньше 0.",
        ));
        parser.register(AddressCommand::new_address(
            0xC000,
            "BR",
            "Присваивает регистру СК значение X",
        ));
        parser.register(AddressCommand::new_address(
            0x1000,
            "AND",
            "Присваивает регистру А результат бинарного И между регистром А и значением в ячейке X",
        ));
        parser.register(AddressCommand::new_address(
            0x4000,
            "ADD",
            "Присваивает регистру А результат сложения регистром А и значением в ячейке X",
        ));
        parser.register(AddressCommand::new_address(
            0x8000,
            "BCS",
            "Присваивает регистру СК значение X, если С равно 1",
        ));
        parser.register(AddressCommand::new_address(0x2000, "JSR", "Команда для организации логики подпрограмм. Значение регистра СК будет положено в ячейку по адресу X после чего регистру СК будет присвоенное значение X + 1"));
        parser.register(AddressCommand::new_address(0x0000, "ISZ", "Увеличивает значение в ячейке по адресу X на 1. После чего, если значение в этой ячейке больше или равно 0, увеличивает СК на 1 тем самым \"перепрыгивает\" следующую команду."));

        parser.register(AddressCommand::new_address(
            0x7000,
            "HZA7",
            "Команда для которой не задано поведение. То есть она ничего не делает.",
        ));
        parser.register(AddressCommand::new_address(
            0xD000,
            "HZAD",
            "Команда для которой не задано поведение. То есть она ничего не делает.",
        ));

        parser.sorted.sort_by(|a, b| {
            let l = format!("{:b}", a.mask())
                .chars()
                .filter(|c| *c == '1')
                .count();
            let r = format!("{:b}", b.mask())
                .chars()
                .filter(|c| *c == '1')
                .count();
            r.cmp(&l)
        });

        parser
    }
}

impl Parser<GeneralCommandInfo> for GeneralParser {
    fn parse(&self, v: u16) -> GeneralCommandInfo {
        for command in &self.sorted {
            if command.matching(v) {
                return GeneralCommandInfo::new(command.clone(), v);
            }
        }
        panic!()
    }

    fn supports_rev_parse(&self) -> bool {
        true
    }

    fn rev_parse(&self, str: &str) -> Result<u16, String> {
        let mnemonic = str.split(' ').next();
        if mnemonic.is_none() {
            return Err("Пустая строка получается".to_string());
        }

        let mnemonic = mnemonic.unwrap().to_uppercase();
        let command = self.mnemonic_map.get(mnemonic.as_str());
        if command.is_none() {
            return Err(format!("Неизвестная мнемоника {mnemonic}"));
        }

        command.unwrap().rev_parse(str)
    }
}

trait GeneralCommand {
    fn matching(&self, cmd: u16) -> bool {
        self.mask().bitand(cmd).bitand(self.mask()) == self.mask()
    }

    fn file_string(&self, cmd: u16) -> String;

    fn mnemonic(&self) -> &str;

    fn mask(&self) -> u16;

    fn parse(&self, data: u16) -> String;

    fn rev_parse(&self, s: &str) -> Result<u16, String>;

    fn draw_highlight(&self, ui: &Ui, opcode: u16);
}

struct SimpleCommand {
    name: &'static str,
    description: &'static str,
    mask: u16,
}

impl SimpleCommand {
    fn new(mask: u16, name: &'static str, description: &'static str) -> SimpleCommand {
        SimpleCommand {
            name,
            description,
            mask,
        }
    }
}

impl GeneralCommand for SimpleCommand {
    fn file_string(&self, cmd: u16) -> String {
        let excessive = cmd.bitand(self.mask.bitxor(0xFFFF));

        if excessive == 0 {
            self.mnemonic().to_owned()
        } else {
            format!("{cmd:0>4X}")
        }
    }

    fn mnemonic(&self) -> &str {
        self.name
    }

    fn mask(&self) -> u16 {
        self.mask
    }

    fn parse(&self, data: u16) -> String {
        if data.bitand(self.mask).bitand(self.mask) != self.mask {
            panic!();
        }

        self.name.to_string()
    }

    fn rev_parse(&self, s: &str) -> Result<u16, String> {
        if s.trim().to_uppercase() != self.name {
            return Err(format!(
                "{} является безадресной командой и не принимает аргументов",
                self.name
            ));
        }
        Ok(self.mask)
    }

    fn draw_highlight(&self, ui: &Ui, opcode: u16) {
        ui.text("Тип: Безадресная команда");
        ui.text(format!("Мнемоника: {}", self.name));
        ui.text(format!("Маска: {:X}", self.mask));

        ui.text_wrapped(format!("Описание: {}", self.description));

        let excessive = opcode.bitand(self.mask.bitxor(0xFFFF));
        if excessive != 0 {
            ui.text_wrapped(format!("Примечание: опкод {:0>4X} был посчитан командой {} так как бинарное и между опкодом {:0>4X} и маской команды {:0>4X} выдало значение равное той же маске {:0>4X}. Таким образом мы просто полностью игнорируем тот факт, что опкод не равен маске.", opcode, self.name, opcode, self.mask, self.mask))
        }
    }
}
struct AddressCommand {
    name: &'static str,
    description: &'static str,
    mask: u16,
    io: bool,
}

impl AddressCommand {
    fn new_address(mask: u16, name: &'static str, description: &'static str) -> AddressCommand {
        AddressCommand {
            name,
            description,
            mask,
            io: false,
        }
    }
    fn new_io(mask: u16, name: &'static str, description: &'static str) -> AddressCommand {
        AddressCommand {
            name,
            description,
            mask,
            io: true,
        }
    }
}

impl GeneralCommand for AddressCommand {
    fn file_string(&self, cmd: u16) -> String {
        self.parse(cmd)
    }

    fn mnemonic(&self) -> &str {
        self.name
    }

    fn mask(&self) -> u16 {
        self.mask
    }

    fn parse(&self, data: u16) -> String {
        if data.bitand(self.mask).bitand(self.mask) != self.mask {
            panic!();
        }

        let address = if !self.io {
            data.bitand(0x7FF)
        } else {
            data.bitand(0xF)
        };
        if data.bitand(0x0800) != 0 {
            format!("{} ({:0>3X})", self.name, address)
        } else {
            format!("{} {:0>3X}", self.name, address)
        }
    }

    fn rev_parse(&self, s: &str) -> Result<u16, String> {
        let splited = s.trim().split(' ').collect::<Vec<&str>>();

        if splited.len() > 2 {
            return Err(format!(
                "Неожиданные штуки:{}",
                splited
                    .iter()
                    .skip(2)
                    .fold("".to_string(), |a, b| format!("{} {}", a, b))
            ));
        }

        if splited.len() < 2 {
            return Err("Ожидалось два параметра".to_string());
        }

        let address = splited.get(1).unwrap().trim();

        let indirect = if !address.is_empty() {
            if address.starts_with('(') {
                if address.chars().nth(address.len() - 1).unwrap() != ')' {
                    return Err("Не закрытая скобка".to_string());
                }
                true
            } else {
                false
            }
        } else {
            false
        };

        let address = if indirect {
            address.get(1..address.len() - 1).unwrap()
        } else {
            address
        };

        if let Ok(parsed) = u16::from_str_radix(address, 16) {
            let max = if self.io { 0xF } else { 0x7FF };
            if parsed > max {
                if self.io {
                    Err("Максимально адресуемое ВУ 0xF".to_string())
                } else {
                    Err("Максимально адресуема память 0x7FF".to_string())
                }
            } else if indirect {
                Ok(self.mask.bitor(parsed).bitor(0x0800))
            } else {
                Ok(self.mask.bitor(parsed))
            }
        } else {
            Err(format!(
                "Ошибка во время парсинга числа {}",
                splited.get(1).unwrap()
            ))
        }
    }

    fn draw_highlight(&self, ui: &Ui, opcode: u16) {
        if self.io {
            ui.text("Тип: Команда ввода-вывода");
        } else {
            ui.text("Тип: Адресная команда");
        }
        ui.text(format!("Мнемоника: {}", self.parse(opcode)));
        ui.text(format!("Маска: {:0>2X}", self.mask));

        let indirect = opcode.bitand(0x0800) != 0;
        let address = opcode.bitand(0x7FF);
        if indirect && !self.io {
            ui.text(format!("X: значение в ячейке {:0>3X}", address))
        } else {
            ui.text(format!("X: {:0>3X}", address))
        }

        ui.text_wrapped(format!("Описание: {}", self.description))
    }
}

pub struct GeneralCommandInfo {
    info: Rc<dyn GeneralCommand>,
    opcode: u16,
}

impl GeneralCommandInfo {
    fn new(info: Rc<dyn GeneralCommand>, opcode: u16) -> GeneralCommandInfo {
        GeneralCommandInfo { info, opcode }
    }
}

impl CommandInfo for GeneralCommandInfo {
    fn file_string(&self) -> String {
        self.info.file_string(self.opcode)
    }

    fn mnemonic(&self) -> String {
        self.info.parse(self.opcode)
    }

    fn draw_highlight(&self, ui: &Ui) {
        self.info.draw_highlight(ui, self.opcode)
    }
}

#[cfg(test)]
mod tests {
    use crate::parse::general::GeneralParser;
    use crate::parse::{CommandInfo, Parser};

    #[test]
    fn test() {
        let parser = GeneralParser::new();

        assert_eq!(parser.parse(0xF700).mnemonic(), "ROR");
        assert_eq!(parser.parse(0x3024).mnemonic(), "MOV 024");
    }
}
