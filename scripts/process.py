import csv
import urllib.request 
import shutil
from pathlib import Path
from textwrap import dedent
from bs4 import BeautifulSoup
import requests

POSTCODES_URL: str = 'https://download.geonames.org/export/zip/allCountries.zip'
COUNTRIES_URL: str = 'https://www.cia.gov/the-world-factbook/references/country-data-codes/'

POSTCODES_PATH: str = 'data/postcodes.csv'
POSTCODES_TEXT: str = 'data/postcodes.txt'

INDEX1_PATH: str = 'data/index/alpha2_to_alpha3.csv'
INDEX2_PATH: str = 'data/index/alpha2_to_code.csv'
INDEX3_PATH: str = 'data/index/code_to_alpha2.csv'

def download_countries(url: str):
    """Download the country data and write it into the output file"""
    response = requests.get(url)
    soup = BeautifulSoup(response.text,features='lxml')

    table = soup.find("table", {"class" : "content-table"})

    alpha2_to_alpha3 = open(INDEX1_PATH,'w')
    alpha2_to_code = open(INDEX2_PATH,'w')
    code_to_alpha2 = open(INDEX3_PATH,'w')

    for row in table.findAll('tr'):
        col = row.findAll('td')

        if len(col) > 2:
            info = col[2]   \
                .string     \
                .strip()    \
                .split('|') 
            
            # get ISO 3166 codes
            if len(info) == 3:

                alpha2 = info[0]
                alpha3 = info[1]
                code = info[2]

                alpha2_to_alpha3.write(f'{alpha2},{alpha3}\n')
                alpha2_to_code.write(f'{alpha2},{code}\n')
                code_to_alpha2.write(f'{code},{alpha2}\n')

    alpha2_to_alpha3.close()
    alpha2_to_code.close()
    code_to_alpha2.close()

def download_postcodes(url: str, file: str):
    """Download the postcode data and unpack it into the output file"""
    name = url.split('/')[-1]  # the name of the data file
    filepath = Path(file)      # the final output path of the data
    dirpath = filepath.parent  # the directory that the output file is in
    tmppath = dirpath / name   # a temporary location for the download

    # the extracted file from the zip
    txtpath = tmppath.with_suffix('.txt')

    # download the zipped data file from the url to the tmp path
    urllib.request.urlretrieve(url,tmppath)

    # extract it into the output directory
    shutil.unpack_archive(tmppath, dirpath)

    # move the extracted file (assumed to be the name from the url with a 
    # .txt extension) to the final output location
    shutil.move(txtpath,filepath)

    # remove the zip file
    tmppath.unlink()

def process_postcodes(txt: str, csv: str):
    """Convert the postcode data to a csv file and clean it up"""
    data = []

    # open the data file for processing
    with open(txt) as f:
        for line in f:

            # remove newlines and escape quotes
            line = line              \
                .strip()             \
                .replace('\n','')    \
                .replace('"','\\"')  \
                .replace('\'','\\\'')
            
            result = []

            # split on tabs and quote column values 
            # that contain commas
            for part in line.split('\t'):
                if ',' in part:
                    result.append(f'"{part}"')
                else:
                    result.append(part)

            # rejoin the parts into a comma-delimited 
            # string
            data.append(','.join(result))

    # write the new csv data out to the csv file
    with open(csv,'w') as f:
        f.write('\n'.join(data))

    Path(txt).unlink(missing_ok=True)

if __name__=='__main__':
    # download_countries(COUNTRIES_URL)
    # download_postcodes(POSTCODES_URL, POSTCODES_TEXT)
    process_postcodes(POSTCODES_TEXT, POSTCODES_PATH)