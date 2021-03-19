// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

/// Given gpubox files, provide a way to output/dump visibilities.
use anyhow::Error;
use mwalib::{misc::get_antennas_from_baseline, CorrelatorContext};
use radix_fmt::radix;
use std::fs::File;
use std::io::Write;
use structopt::StructOpt;

#[cfg(not(tarpaulin_include))]
#[derive(StructOpt, Debug)]
#[structopt(name = "mwalib-data-dump", author)]
pub struct DumpAllDataOpt {
    /// Path to the metafits file.
    #[structopt(short, long, parse(from_os_str))]
    pub metafits: std::path::PathBuf,

    /// Paths to the gpubox files.
    #[structopt(name = "GPUBOX FILE", parse(from_os_str))]
    pub files: Vec<std::path::PathBuf>,

    // Dump filename
    #[structopt(short, long, parse(from_os_str))]
    pub dump_filename: std::path::PathBuf,

    // Radix (base) of visibility values
    #[structopt(short, long, default_value = "0")]
    pub vis_radix: u8,

    // Dump absolute of float values
    #[structopt(short, long)]
    pub absolute: bool,
}

#[cfg(not(tarpaulin_include))]
pub fn dump_all_data<T: AsRef<std::path::Path>>(
    metafits: &T,
    files: &[T],
    dump_filename: &T,
    vis_radix: u8,
    absolute: bool,
) -> Result<(), Error> {
    let mut dump_file = File::create(dump_filename)?;
    println!("Dumping data via mwalib...");
    let mut context = CorrelatorContext::new(metafits, files)?;
    let coarse_channel_array = context.coarse_channels.clone();
    let timestep_array = context.timesteps.clone();

    println!("Correlator version: {}", context.corr_version);

    let floats_per_finechan = context.metafits_context.num_visibility_pols * 2;
    let floats_per_baseline =
        context.metafits_context.num_fine_channels_per_coarse * floats_per_finechan;

    let mut sum: f64 = 0.;
    let mut float_count: u64 = 0;
    writeln!(
        &mut dump_file,
        "coarse_chan,timestep,baseline,ant1_name,ant2_name,fine_chan,xx_re,xx_im,xy_re,xy_im,yx_re,yx_im,yy_re,yy_im"
    )?;
    for (coarse_channel_index, coarse_channel) in coarse_channel_array.iter().enumerate() {
        for (timestep_index, timestep) in timestep_array.iter().enumerate() {
            println!(
                "Reading coarse chan: {} ({}) {:.3} Mhz, timestep {} ({:?})",
                coarse_channel_index,
                coarse_channel.receiver_channel_number,
                coarse_channel.channel_centre_hz as f32 / 1.0e6,
                timestep_index,
                timestep
            );
            let img_buffer = context.read_by_baseline(timestep_index, coarse_channel_index)?;

            for (baseline_index, baseline_chunk) in
                img_buffer.chunks(floats_per_baseline).enumerate()
            {
                let (ant1, ant2) = get_antennas_from_baseline(
                    baseline_index,
                    context.metafits_context.num_antennas,
                )
                .unwrap();
                let ant1_name: String = context.metafits_context.antennas[ant1]
                    .tile_name
                    .to_string();
                let ant2_name: String = context.metafits_context.antennas[ant2]
                    .tile_name
                    .to_string();

                for (fine_chan_index, fine_chan_chunk) in
                    baseline_chunk.chunks(floats_per_finechan).enumerate()
                {
                    write!(
                        &mut dump_file,
                        "{},{},{},{},{},{}",
                        coarse_channel_index,
                        timestep_index,
                        baseline_index,
                        ant1_name,
                        ant2_name,
                        fine_chan_index,
                    )?;
                    for &float_val in fine_chan_chunk {
                        let abs_val = if absolute { float_val.abs() } else { float_val };
                        if vis_radix > 0 {
                            let radix_val = radix(
                                abs_val as u64,
                                vis_radix,
                            );
                            write!(&mut dump_file, ",{}", radix_val)?;
                        } else {
                            write!(&mut dump_file, ",{}", abs_val)?;
                        }
                        println!("{:08}", abs_val);
                        sum += float_val as f64;
                        float_count += 1;
                    }
                    writeln!(&mut dump_file)?;
                }
            }
        }
    }

    println!("Sum was {}, count was {} floats", sum, float_count);

    Ok(())
}
