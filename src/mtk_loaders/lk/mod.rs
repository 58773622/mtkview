use std::ops::Range;

use binaryninja::{
    binary_view::{BinaryViewEventHandler, BinaryViewExt},
    data_buffer::DataBuffer,
    section::SectionBuilder,
};
use tracing::{debug, info, warn};

use crate::{BinaryViewResult, util::read_data_slice_u32};

//pub(crate) mod lk_headers;
//pub(crate) mod lk_types;
//pub(crate) mod view;

struct RomPairs {
    header: Vec<u8>,
    data: Vec<u8>,
}

pub(crate) struct MTKLkLoader {}

impl MTKLkLoader {
    pub fn new(data: DataBuffer) -> BinaryViewResult<Self> {
        let image_data = data.get_data();

        todo!()
    }
}

pub(crate) struct LkMd1RomHookContext {
    entry_point: u32,
    data_base_address: u32,
    done: bool,
}

impl Default for LkMd1RomHookContext {
    fn default() -> Self {
        Self {
            entry_point: 0,
            data_base_address: 0,
            done: false,
        }
    }
}

impl LkMd1RomHookContext {
    pub fn new(entry_point: u32, data_base_address: u32) -> Self {
        Self {
            entry_point,
            data_base_address,
            done: false,
        }
    }
}

impl BinaryViewEventHandler for LkMd1RomHookContext {
    fn on_event(&self, binary_view: &binaryninja::binary_view::BinaryView) {
        if self.done {
            return;
        }
        let Some(pv) = binary_view.parent_view() else {
            debug!("waiting for parent view..");
            return;
        };
        let Some(lk_code) = pv.section_by_name("lk_data") else {
            debug!("MD1ROM Has no LK.");
            return;
        };
        let lk_code_address_range = lk_code.address_range();
        if lk_code.len() == 0 {
            return;
        }

        println!(
            "lk_data: {:#X} - {:#X}",
            lk_code_address_range.start, lk_code_address_range.end
        );

        let lk_code_data = {
            let b = pv
                .read_buffer(lk_code_address_range.start, lk_code.len())
                .unwrap();
            b.get_data().to_owned()
        };
        println!("SIZE: {}", lk_code_data.len());
        let code_base_addr = read_data_slice_u32(&lk_code_data, 0x74).unwrap();
        info!("Got load base addr: {:#X}", code_base_addr);

        let new_sec_range = Range {
            start: code_base_addr as u64,
            end: lk_code_data.len() as u64 + code_base_addr as u64,
        };

        let section_builder = SectionBuilder::new("LK Code".to_string(), new_sec_range);
        pv.remove_auto_section(lk_code.name());
        pv.add_section(section_builder);
        //binary_view.add_section();
    }
}
