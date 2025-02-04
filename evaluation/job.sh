#!/bin/sh
#SBATCH --ntasks=1
#SBATCH --cpus-per-task=1
#SBATCH --time=30
#SBATCH --mem=5gb
#SBATCH --partition=single

apptainer run --env TMPDIR="${PWD}/tmp" docker://ghcr.io/uulm-janbaudisch/cnf_partitioner:main-amd64 --blocks 2 $1
