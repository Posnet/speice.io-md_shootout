#!/usr/bin/env python3
import pandas as pd

def parse_block(lines, run_format, run_date):
    # Future reference: output benchmark results in a format
    # that doesn't require extra code to read
    return {
        'run_date': run_date,
        'run_format' : run_format,
        'protocol' : lines[0].split(' total')[0],
        'total_secs' : int(lines[0].split('=')[1][:-2]),
        'serialize_50_nanos' : int(lines[2].split('=')[1][:-3]),
        'serialize_99_nanos' : int(lines[3].split('=')[1][:-3]),
        'serialize_999_nanos' : int(lines[4].split('=')[1][:-3]),
        'deserialize_50_nanos' : int(lines[5].split('=')[1][:-3]),
        'deserialize_99_nanos' : int(lines[6].split('=')[1][:-3]),
        'deserialize_999_nanos' : int(lines[7].split('=')[1][:-3]),
        'serialize_total_nanos' : int(lines[8].split('=')[1][:-3]),
        'deserialize_total_nanos' : int(lines[9].split('=')[1][:-3]),
    }


def main(filename: str, run_format: str):
    records = []
    run_count = 10
    
    with open(filename, 'r') as handle:
        lines = handle.readlines()

    for i in range(run_count):
        num_blocks = 4
        block_len = 12
        run_date = lines[i * num_blocks * block_len + i].split('_')[2]

        for block in range(num_blocks):
            lower_block = i * block_len * num_blocks + block * block_len + i + 1
            upper_block = i * block_len * num_blocks + block * block_len + block_len + i + 1

            inner_lines = lines[lower_block:upper_block]
            rec = parse_block(inner_lines, run_format, run_date)
            records.append(rec)

    return records


if __name__ == '__main__':
    all_records = []

    runs = [
        ('shootout_normal.txt', ''),
        ('shootout_taskset.txt', ''),
        ('shootout_nice.txt', '')
    ]
    for fname, run_format in runs:
        for record in main(fname, run_format):
            all_records.append(record)

    pd.DataFrame.from_records(all_records).to_csv('shootout.csv')
