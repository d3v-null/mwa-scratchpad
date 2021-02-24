// use chrono::{DateTime, FixedOffset};
use mwalib::{
    // antenna::Antenna,
    // baseline::Baseline,
    coarse_channel::CoarseChannel,
    metafits_context::MetafitsContext,
    // conversion::LegacyConversionBaseline,
    correlator_context::CorrelatorVersion,
    // gpubox_files::GPUBoxBatch,
    // rfinput::RFInput,
    timestep::TimeStep,
    // visibility_pol::VisibilityPol,
    CorrelatorContext,
};
use serde::Serialize;
use std::collections::BTreeMap;

#[derive(Serialize, Debug)]
#[serde(remote = "CorrelatorContext")]
pub struct MWALibContextDef {
    // pub mwa_latitude_radians: f64,
    // pub mwa_longitude_radians: f64,
    // pub mwa_altitude_metres: f64,
    // pub coax_v_factor: f64,
    // pub obsid: u32,
    // pub scheduled_start_gpstime_milliseconds: u64,
    // pub scheduled_end_gpstime_milliseconds: u64,
    // pub scheduled_start_unix_time_milliseconds: u64,
    // pub scheduled_end_unix_time_milliseconds: u64,
    // #[serde(with = "date_time_fixed_offset_shim")]
    // pub scheduled_start_utc: DateTime<FixedOffset>,
    // #[serde(with = "date_time_fixed_offset_shim")]
    // pub scheduled_end_utc: DateTime<FixedOffset>,
    // pub scheduled_start_mjd: f64,
    // pub scheduled_end_mjd: f64,
    // pub scheduled_duration_milliseconds: u64,
    // pub ra_tile_pointing_degrees: f64,
    // pub dec_tile_pointing_degrees: f64,
    // pub ra_phase_center_degrees: Option<f64>,
    // pub dec_phase_center_degrees: Option<f64>,
    // pub azimuth_degrees: f64,
    // pub altitude_degrees: f64,
    // pub sun_altitude_degrees: f64,
    // pub sun_distance_degrees: f64,
    // pub moon_distance_degrees: f64,
    // pub jupiter_distance_degrees: f64,
    // pub lst_degrees: f64,
    // pub hour_angle_string: String,
    // pub grid_name: String,
    // pub grid_number: i32,
    // pub creator: String,
    // pub project_id: String,
    // pub observation_name: String,
    // pub mode: String,
    // pub receivers: Vec<usize>,
    // pub dipole_delays: Vec<usize>,
    // pub global_analogue_attenuation_db: f64,
    // pub quack_time_duration_milliseconds: u64,
    // pub good_time_unix_milliseconds: u64,
    #[serde(with = "correlator_version_shim")]
    pub corr_version: CorrelatorVersion,
    #[serde(skip)]
    pub metafits_context: MetafitsContext,
    pub start_unix_time_milliseconds: u64,
    pub end_unix_time_milliseconds: u64,
    pub duration_milliseconds: u64,
    pub num_timesteps: usize,
    #[serde(with = "time_steps_shim")]
    pub timesteps: Vec<TimeStep>,
    // pub num_antennas: usize,
    // #[serde(with = "antennas_shim")]
    // pub antennas: Vec<Antenna>,
    // pub num_baselines: usize,
    // #[serde(with = "baselines_shim")]
    // pub baselines: Vec<Baseline>,
    // pub num_rf_inputs: usize,
    // #[serde(with = "rfinputs_shim")]
    // pub rf_inputs: Vec<RFInput>,
    // pub num_antenna_pols: usize,
    // pub num_visibility_pols: usize,
    // #[serde(with = "visibility_pols_shim")]
    // pub visibility_pols: Vec<VisibilityPol>,
    // pub correlator_integration_time_milliseconds: u64,
    pub num_coarse_channels: usize,
    #[serde(with = "coarse_channels_shim")]
    pub coarse_channels: Vec<CoarseChannel>,
    pub bandwidth_hz: u32,
    // pub coarse_channel_width_hz: u32,
    // pub correlator_fine_channel_width_hz: u32,
    // pub metafits_centre_freq_hz: u32,
    // pub num_fine_channels_per_coarse: usize,
    // pub metafits_filename: String,
    // #[serde(with = "gpu_box_batch_shim")]
    // pub gpubox_batches: Vec<GPUBoxBatch>,
    #[serde(skip)]
    pub gpubox_time_map: BTreeMap<u64, BTreeMap<usize, (usize, usize)>>,
    pub num_timestep_coarse_channel_bytes: usize,
    pub num_timestep_coarse_channel_floats: usize,
    pub num_gpubox_files: usize,
    // #[serde(skip)]
    // pub legacy_conversion_table: Vec<LegacyConversionBaseline>,
}

// mod date_time_fixed_offset_shim {
//     use chrono::{DateTime, FixedOffset};
//     use serde::Serializer;

//     pub fn to_serializable(v: &DateTime<FixedOffset>) -> String {
//         v.to_rfc3339()
//     }

//     pub fn serialize<S>(v: &DateTime<FixedOffset>, s: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         s.serialize_str(&to_serializable(v))
//     }
// }

mod correlator_version_shim {
    use mwalib::correlator_context::CorrelatorVersion;
    use serde::Serializer;

    pub fn to_serializable(v: &CorrelatorVersion) -> &str {
        match v {
            CorrelatorVersion::Legacy => "Legacy",
            CorrelatorVersion::V2 => "V2",
            CorrelatorVersion::OldLegacy => "OldLegacy",
        }
    }

    pub fn serialize<S>(v: &CorrelatorVersion, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        s.serialize_str(to_serializable(v))
    }
}

mod time_steps_shim {
    use mwalib::timestep::TimeStep;
    use serde::{ser::SerializeSeq, Serializer};

    pub fn to_serializable(v: &Vec<TimeStep>) -> Vec<u64> {
        v.into_iter().map(|e| e.unix_time_milliseconds).collect()
    }

    pub fn serialize<S>(v: &Vec<TimeStep>, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = s.serialize_seq(Some(v.len()))?;
        for e in to_serializable(v) {
            seq.serialize_element(&e)?;
        }
        seq.end()
    }
}

// mod visibility_pols_shim {
//     use mwalib::visibility_pol::VisibilityPol;
//     use serde::{ser::SerializeSeq, Serializer};

//     pub fn to_serializable(v: &Vec<VisibilityPol>) -> Vec<String> {
//         v.into_iter().map(|e| e.polarisation.clone()).collect()
//     }

//     pub fn serialize<S>(v: &Vec<VisibilityPol>, s: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         let mut seq = s.serialize_seq(Some(v.len()))?;
//         for e in to_serializable(v) {
//             seq.serialize_element(&e)?;
//         }
//         seq.end()
//     }
// }

mod pol_shim {
    use mwalib::rfinput::Pol;
    use serde::Serializer;

    pub fn to_serializable(v: &Pol) -> &str {
        match v {
            Pol::X => "X",
            Pol::Y => "Y",
        }
    }

    pub fn serialize<S>(v: &Pol, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        s.serialize_str(&to_serializable(v))
    }
}

mod rfinput_shim {
    use super::pol_shim;
    use mwalib::rfinput::{Pol, RFInput};
    use serde::{ser::SerializeStruct, Serialize, Serializer};

    #[serde(remote = "RFInput")]
    #[derive(Serialize, Debug, Clone)]
    pub struct MWALibRFInputDef {
        pub input: u32,
        pub antenna: u32,
        pub tile_id: u32,
        pub tile_name: String,
        #[serde(with = "pol_shim")]
        pub pol: Pol,
        pub electrical_length_m: f64,
        pub north_m: f64,
        pub east_m: f64,
        pub height_m: f64,
        pub vcs_order: u32,
        pub subfile_order: u32,
        pub flagged: bool,
        pub digital_gains: Vec<u32>,
        pub dipole_delays: Vec<u32>,
        pub receiver_number: u32,
        pub receiver_slot_number: u32,
    }

    #[derive(Serialize)]
    pub struct MWALibRFInputWrapper(#[serde(with = "MWALibRFInputDef")] pub RFInput);

    pub fn serialize<S>(v: &RFInput, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = s.serialize_struct("MWALibRFInputWrapper", 15)?;
        state.serialize_field("input", &v.input)?;
        state.serialize_field("antenna", &v.antenna)?;
        state.serialize_field("tile_id", &v.tile_id)?;
        state.serialize_field("tile_name", &v.tile_name)?;
        state.serialize_field("pol", pol_shim::to_serializable(&v.pol))?;
        state.serialize_field("electrical_length_m", &v.electrical_length_m)?;
        state.serialize_field("north_m", &v.north_m)?;
        state.serialize_field("east_m", &v.east_m)?;
        state.serialize_field("height_m", &v.height_m)?;
        state.serialize_field("vcs_order", &v.vcs_order)?;
        state.serialize_field("subfile_order", &v.subfile_order)?;
        state.serialize_field("flagged", &v.flagged)?;
        state.serialize_field("digital_gains", &v.digital_gains)?;
        state.serialize_field("dipole_delays", &v.dipole_delays)?;
        state.serialize_field("receiver_number", &v.receiver_number)?;
        state.serialize_field("receiver_slot_number", &v.receiver_slot_number)?;
        state.end()
    }
}

// mod rfinputs_shim {
//     use super::rfinput_shim;
//     use mwalib::rfinput::RFInput;
//     use serde::{ser::SerializeSeq, Serializer};

//     pub fn serialize<S>(v: &Vec<RFInput>, s: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         let mut seq = s.serialize_seq(Some(v.len()))?;
//         for e in v {
//             seq.serialize_element(&rfinput_shim::MWALibRFInputWrapper(e.clone()))?;
//         }
//         seq.end()
//     }
// }

// mod antennas_shim {
//     use super::rfinput_shim;
//     use mwalib::{antenna::Antenna, rfinput::RFInput};
//     use serde::{ser::SerializeSeq, Serialize, Serializer};

//     #[serde(remote = "Antenna")]
//     #[derive(Serialize, Debug)]
//     pub struct MWALibAntennaDef {
//         pub antenna: u32,
//         pub tile_id: u32,
//         pub tile_name: String,
//         #[serde(with = "rfinput_shim")]
//         pub x_pol: RFInput,
//         #[serde(with = "rfinput_shim")]
//         pub y_pol: RFInput,
//     }

//     #[derive(Serialize)]
//     pub struct MWALibAntennaWrapper(#[serde(with = "MWALibAntennaDef")] Antenna);

//     pub fn serialize<S>(v: &Vec<Antenna>, s: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         let mut seq = s.serialize_seq(Some(v.len()))?;
//         for e in v {
//             seq.serialize_element(&MWALibAntennaWrapper(e.clone()))?;
//         }
//         seq.end()
//     }
// }

// mod baselines_shim {
//     use mwalib::baseline::Baseline;
//     use serde::{ser::SerializeSeq, Serialize, Serializer};

//     // #[serde(remote = "Baseline")]
//     #[derive(Serialize, Debug)]
//     pub struct MWALibBaselineDef(usize, usize);

//     pub fn serialize<S>(v: &Vec<Baseline>, s: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         let mut seq = s.serialize_seq(Some(v.len()))?;
//         for e in v {
//             seq.serialize_element(&MWALibBaselineDef(e.antenna1_index, e.antenna2_index))?;
//         }
//         seq.end()
//     }
// }

mod coarse_channels_shim {
    use mwalib::coarse_channel::CoarseChannel;
    use serde::{ser::SerializeSeq, Serialize, Serializer};

    #[serde(remote = "CoarseChannel")]
    #[derive(Serialize, Debug)]
    pub struct MWALibCoarseChannelDef {
        pub correlator_channel_number: usize,
        pub receiver_channel_number: usize,
        pub gpubox_number: usize,
        pub channel_width_hz: u32,
        pub channel_start_hz: u32,
        pub channel_centre_hz: u32,
        pub channel_end_hz: u32,
    }

    #[derive(Serialize)]
    pub struct MWALibCoarseChannelWrapper(#[serde(with = "MWALibCoarseChannelDef")] CoarseChannel);

    pub fn serialize<S>(v: &Vec<CoarseChannel>, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = s.serialize_seq(Some(v.len()))?;
        for e in v {
            seq.serialize_element(&MWALibCoarseChannelWrapper(e.clone()))?;
        }
        seq.end()
    }
}

mod gpu_box_file_shim {
    use mwalib::gpubox_files::GPUBoxFile;
    use serde::{ser::SerializeSeq, Serialize, Serializer};

    #[serde(remote = "GPUBoxFile")]
    #[derive(Serialize, Debug)]
    pub struct GPUBoxFileDef {
        pub filename: String,
        pub channel_identifier: usize,
    }

    impl From<GPUBoxFile> for GPUBoxFileDef {
        fn from(file: GPUBoxFile) -> Self {
            GPUBoxFileDef {
                filename: file.filename.clone(),
                channel_identifier: file.channel_identifier,
            }
        }
    }

    #[derive(Serialize)]
    pub struct GPUBoxBatchWrapper(#[serde(with = "GPUBoxFileDef")] GPUBoxFile);

    pub fn serialize<S>(v: &Vec<GPUBoxFile>, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = s.serialize_seq(Some(v.len()))?;
        for e in v {
            seq.serialize_element(&GPUBoxBatchWrapper(GPUBoxFile {
                filename: e.filename.clone(),
                channel_identifier: e.channel_identifier,
            }))?;
        }
        seq.end()
    }
}

// pub mod gpu_box_batch_shim {
//     use super::gpu_box_file_shim;
//     use mwalib::gpubox_files::{GPUBoxBatch, GPUBoxFile};
//     use serde::{ser::SerializeSeq, Serialize, Serializer};

//     #[serde(remote = "GPUBoxBatch")]
//     #[derive(Serialize, Debug)]
//     pub struct GPUBoxBatchDef {
//         pub batch_number: usize,
//         #[serde(with = "gpu_box_file_shim")]
//         pub gpubox_files: Vec<GPUBoxFile>,
//     }

//     #[derive(Serialize)]
//     pub struct GPUBoxBatchWrapper(#[serde(with = "GPUBoxBatchDef")] GPUBoxBatch);

//     pub fn to_serializable(batch: &GPUBoxBatch) -> GPUBoxBatchWrapper {
//         GPUBoxBatchWrapper(GPUBoxBatch {
//             batch_number: batch.batch_number,
//             gpubox_files: batch
//                 .gpubox_files
//                 .iter()
//                 .map(|f| GPUBoxFile {
//                     filename: f.filename.clone(),
//                     channel_identifier: f.channel_identifier,
//                 })
//                 .collect(),
//         })
//     }

//     pub fn serialize<S>(v: &Vec<GPUBoxBatch>, s: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         let mut seq = s.serialize_seq(Some(v.len()))?;
//         for e in v {
//             seq.serialize_element(&to_serializable(e))?;
//         }
//         seq.end()
//     }
// }

#[derive(Serialize)]
pub struct SerializableContext(#[serde(with = "MWALibContextDef")] CorrelatorContext);

pub fn serialize_context(context: CorrelatorContext) -> SerializableContext {
    SerializableContext(context)
}
