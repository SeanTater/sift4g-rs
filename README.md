# SIFT4G

SIFT (Sorting Intolerant From Tolerant) For Genomes (http://www.nature.com/nprot/journal/v11/n1/full/nprot.2015.123.html)

## Requirements
- g++ (4.\*+)
- GNU Make

\*note: It was only tested on Linux (Ubuntu 14.04).

## Installation

To build SIFT4G run the following commands from your terminal:

    git clone --recursive https://github.com/rvaser/sift4g.git sift4g
    cd sift4g/
    make

Running the 'make' command will create the bin folder which contains the sift4g executable. If you left out '--recursive' from git clone, after cloning run the following commands:

    git submodule init
    git submodule update

## Usage

To run the default version of SIFT4G run the following command:

    ./sift4g -i <query file> -j <database file>

To see all available parameters run the command bellow:

    ./sift4g -h
