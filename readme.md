# i.MX ROM log decoder
This project is meant to help decode the ROM logs from an i.MX device, for details please refer to [AN12853](https://www.nxp.com/webapp/Download?colCode=AN12853).

## Retrieving the logs
The attached (imx8qx\_dump\_rom\_logs.cmm) lauterbach scripts can be used to generate the ROM logs from a device.
The scripts will create a file called 'rom\_logs.txt' in the following format:
```
E0000000
01000001
1F000003
12000000
22000000
50000000
```

## Using the decoder

The decoder is written in rust, to build from source a rust compiler is required, please see the [official rust documents](https://www.rust-lang.org/tools/install) for details on how to do this.
Once installed you can simple run:
> cargo build --release

A binary is provided for x86\_64\_linux.

Simply call as follows:
> ./rom\_log\_decoder rom\_log.txt decoded\_log.txt

The decoded\_log.txt will contain the decoded ROM logs in the following format:
```
Event ID:E0-Internal use, 0 parameters
E0000000

Event ID:01-ROM event version, bit[7:0] is the version, 0 parameters
01000001

Event ID:1F-Raw boot mode setting in OCOTP fuses Bit[23:0] == Raw boot mode setting, 0 parameters
1F000003

Event ID:12-Boot mode is Internal Boot, 0 parameters
12000000

Event ID:22-Secure config is open, 0 parameters
22000000

Event ID:50-Boot from the primary boot image, 0 parameters
50000000
```
