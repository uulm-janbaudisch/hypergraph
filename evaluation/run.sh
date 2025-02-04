#!/bin/sh

rm -rf tmp
mkdir tmp

mkdir -p output

files=$(find $1 -type f)

for file in $files
do
  filename=$(basename $file)
  problem="${filename%.*}"
  sbatch --job-name $problem --output "output/${problem}.out" --error "output/${problem}.err" job.sh $file
done
