// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

/// Given gpubox files, provide a way to output/dump visibilities.
use anyhow::*;
use mwalib::*;
use structopt::StructOpt;

mod dump_context;
use dump_context::DumpContextOpt;

mod dump_data;
use dump_data::DumpDataOpt;

#[derive(StructOpt, Debug)]
enum Args {
    DumpContext(DumpContextOpt),
    DumpData(DumpDataOpt),
}

fn main() -> Result<(), anyhow::Error> {
    match Args::from_args() {
        Args::DumpContext(DumpContextOpt { metafits, files }) => {
            let mut context = mwalibContext::new(&metafits, &files)?;
            context.rf_inputs.sort_by_key(|k| k.subfile_order);

            println!("{}", context);

            Ok(())
        }
        Args::DumpData(DumpDataOpt {
            timestep,
            baseline,
            fine_chan1,
            fine_chan2,
            coarse_channel,
            metafits,
            files,
            dump_filename,
        }) => {
            dump_data::dump_data(
                &metafits,
                &files,
                timestep,
                baseline,
                (fine_chan1, fine_chan2),
                coarse_channel,
                &dump_filename,
            )?;

            Ok(())
        }
    }
}
