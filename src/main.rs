// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

/// Given gpubox files, provide a way to output/dump visibilities.
use anyhow::Error;
use structopt::StructOpt;

mod dump_all_data;
mod serialize;
use dump_all_data::DumpAllDataOpt;

#[derive(StructOpt, Debug)]
enum Args {
    DumpAllData(DumpAllDataOpt),
}

fn main() -> Result<(), Error> {
    match Args::from_args() {
        Args::DumpAllData(DumpAllDataOpt {
            metafits,
            files,
            dump_filename,
        }) => {
            dump_all_data::dump_all_data(&metafits, &files, &dump_filename)?;

            Ok(())
        }
    }
}
