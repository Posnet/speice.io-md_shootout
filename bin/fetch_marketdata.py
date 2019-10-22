#!/bin/env python3

import gzip
import shutil
import argparse
import json
import sys
import gzip
import os
from datetime import datetime, timedelta
from urllib.request import urlopen, urlretrieve
from urllib.parse import unquote
from multiprocessing.pool import Pool

BASE_URL = "https://api.iextrading.com/1.0/hist"
CONCURRENT_DOWNLOAD_LIMIT = 8
TARGET_DIR = 'data'

def get_filename(url):
    base, params = url.split('?')
    fname = base.rsplit('/', 1)[-1]
    fname = unquote(fname)
    fname = fname.replace('/', '_')
    fname = TARGET_DIR + '/' + fname
    return fname

def fetch_url(url):
    try:
        fname = get_filename(url)
        fname, _ = urlretrieve(url, fname)
        print("Fetched:", fname)
        target = fname.rsplit('.', 1)[0]
        with gzip.open(fname, 'rb') as f_in:
            with open(target, 'wb') as f_out:
                shutil.copyfileobj(f_in, f_out)
        print("Decompressed:", target)
    finally:
        os.remove(fname)

def main():
    parser = argparse.ArgumentParser(description='Fetch IEX Historical market data.')
    parser.add_argument('-s', '--start-date', type=int, default=20190903, help='Start date in ordinal format e.g. 20190903')
    parser.add_argument('-e', '--end-date', type=int, default=20190906, help='End date in ordinal format e.g. 20190906')
    args = parser.parse_args()

    d1 = datetime.strptime(str(args.start_date), '%Y%m%d')
    d2 = datetime.strptime(str(args.end_date), '%Y%m%d')

    days = [d1 + timedelta(days=x) for x in range((d2-d1).days + 1)]
    days = [d.strftime('%Y%m%d') for d in days]

    print("Fetching Urls for:", days)

    urls = []
    for day in days:
        with urlopen(BASE_URL + '?date=' + str(day)) as resp:
            data = resp.read().decode('ascii')
            data = json.loads(data)
            for feed in data:
                if feed['feed'] == 'DEEP':
                    urls.append(feed['link'])


    print("Fetching market data...")
    pool = Pool(max(len(urls),CONCURRENT_DOWNLOAD_LIMIT)) # Limit to 8 concurrent downloads
    results = pool.imap_unordered(fetch_url, urls)
    pool.close()
    pool.join()
    print("Done")

if __name__ == '__main__':
    main()
