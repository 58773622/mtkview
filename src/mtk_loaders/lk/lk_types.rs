use binaryninja::{
    platform::Platform,
    types::{CoreTypeParser, ParsedType, TypeParser, TypeParserResult},
};

pub(crate) const LK_TYPES_C_SRC: &'static str = r#"
/* LK image header */

union lk_hdr {
	struct {
		uint32_t magic;
		uint32_t size;
		char name[32];
		uint32_t loadaddr;
		uint32_t mode;
	};

	uint8_t data[512];
};

#define LK_PART_MAGIC		0x58881688
"#;

pub struct LkCPlatformTypes {
    parsed_types: TypeParserResult,
}

impl LkCPlatformTypes {
    pub fn new(platform_str: &str) -> Self {
        let platform = Platform::by_name(platform_str).unwrap();
        let plat_type_container = platform.type_container();
        let type_parser = CoreTypeParser::default();
        let parsed_types = type_parser
            .parse_types_from_source(
                LK_TYPES_C_SRC,
                "lk_types.h",
                &platform,
                &plat_type_container,
                &[],
                &[],
                "",
            )
            .unwrap();

        Self { parsed_types }
    }

    pub fn get_type_by_name(&self, name: &str) -> Option<ParsedType> {
        for t in &self.parsed_types.types {
            if t.name == name.into() {
                return Some(t.clone());
            }
        }
        None
    }

    pub fn get_all_types(&self) -> Option<Vec<ParsedType>> {
        if !self.parsed_types.types.is_empty() {
            return Some(self.parsed_types.types.clone());
        }
        None
    }
}
