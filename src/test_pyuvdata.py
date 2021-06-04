import os
from pyuvdata import UVData
UV = UVData()

# DATA_PATH = os.path.join("../Birli/tests/data/1196175296_mwa_ord/")
# DATA_PATH = os.path.join("../mwalib/test_files/1101503312_1_timestep/")
DATA_PATH = os.path.join("/mnt/c/Documents and Settings/derwe/CIRA/data/1196175296_vis/")

def main():
    filelist = [DATA_PATH + i for i in [
        '1196175296.metafits',
        '1196175296_20171201145440_gpubox01_00.fits',
        '1196175296_20171201145440_gpubox02_00.fits',
    ]]
    # filelist = [DATA_PATH + i for i in [
    #     '1101503312.metafits',   
    #     '1101503312_20141201210818_gpubox01_00.fits',
    # ]]
    # filelist = [DATA_PATH + i for i in [
    #     '1196175296.metafits',   
    #     "1196175296_20171201145440_gpubox02_00.fits",  
    #     "1196175296_20171201145540_gpubox02_01.fits",  
    #     "1196175296_20171201145440_gpubox01_00.fits",  
    #     "1196175296_20171201145540_gpubox01_01.fits",  
    # ]]

    UV.read_mwa_corr_fits(
        filelist, 
        correct_cable_len=False, 
        phase_to_pointing_center=False, 
        flag_init=False
    )

    out_file = os.path.join('.', 'tutorial.uvfits')
    UV.write_uvfits(out_file, spoof_nonessential=True, force_phase=True)

    
if __name__ == "__main__":
    main()