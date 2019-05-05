extern crate nearfield_sys;

use modulation::Modulation;
use modulation_type::ModulationType;
use std::fmt;

pub(crate) type InternalTarget = nearfield_sys::nfc_target;
pub type TargetInfo = nearfield_sys::nfc_target_info;

pub struct Target {
    pub modulation: Modulation,
    info: TargetInfo,
}

impl Target {
    pub(crate) fn from_sys(internal: InternalTarget) -> Target {
        let modulation = Modulation::from_sys(internal.nm);

        Target {
            modulation,
            info: internal.nti,
        }
    }

    pub(crate) fn to_sys(&self) -> InternalTarget {
        InternalTarget {
            nm: self.modulation.to_sys(),
            nti: self.info,
        }
    }
}

impl fmt::Display for Target {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output = String::new();

        output.push_str(&format!("{} target:\n", self.modulation));

        match self.modulation.modulation_type {
            ModulationType::ISO14443A => {
                let info = unsafe { self.info.nai };

                output.push_str(&format!("\tATQA (SENS_RES): {:?}\n", info.abtAtqa));
                output.push_str(&format!("\tUID (NFCID1): {:?}\n", info.btSak));
                output.push_str(&format!("\tSAK (SEL_RES): {:?}\n", info.abtUid));
            }
            _ => output.push_str("\tModulation type not supported"),
        }

        write!(f, "{}", output)
    }
}
