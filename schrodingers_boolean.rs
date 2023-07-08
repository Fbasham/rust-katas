#[derive(Debug)]
struct OmniBool {}

impl PartialEq<OmniBool> for OmniBool {
    fn eq(&self, other: &Self) -> bool {
        true
    }
}

impl PartialEq<bool> for OmniBool {
    fn eq(&self, other: &bool) -> bool {
        true
    }
}

impl Eq for OmniBool {}

const omnibool: OmniBool = OmniBool {};
