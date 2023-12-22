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
POSTCODES_CODE: str = 'src/codes/'

INDEX1_PATH: str = 'data/index/alpha2_to_alpha3.csv'
INDEX2_PATH: str = 'data/index/alpha2_to_code.csv'
INDEX3_PATH: str = 'data/index/code_to_alpha2.csv'

def download_countries(url: str):
    """Download the country data and write it into the output file"""
    Path(INDEX1_PATH).parent.mkdir(exist_ok=True)
    Path(INDEX2_PATH).parent.mkdir(exist_ok=True)
    Path(INDEX3_PATH).parent.mkdir(exist_ok=True)

    response = requests.get(url)
    soup = BeautifulSoup(response.text,features='lxml')

    table = soup.find("table", {"class" : "content-table"})

    alpha2_to_alpha3 = open(INDEX1_PATH,'w')
    alpha2_to_code = open(INDEX2_PATH,'w')
    code_to_alpha2 = open(INDEX3_PATH,'w')

    # manually add this one because it's not in the list
    # for some reason
    alpha2_to_alpha3.write(f'AX,ALA\n')

    alpha2_to_alpha3_dict = {}
    alpha2_to_code_dict = {}
    code_to_alpha2_dict = {}

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

                alpha2_to_alpha3_dict[alpha2] = alpha3
                alpha2_to_code_dict[alpha2] = code
                code_to_alpha2_dict[code] = alpha2

    for key, value in alpha2_to_alpha3_dict.items():
        alpha2_to_alpha3.write(f'{key},{value}\n')

    for key, value in alpha2_to_code_dict.items():
        alpha2_to_code.write(f'{key},{value}\n')

    for key, value in code_to_alpha2_dict.items():
        code_to_alpha2.write(f'{key},{value}\n')

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

def process_postcodes(txtpath: str, csvpath: str):
    """Convert the postcode data to a csv file and clean it up"""
    data = []

    input_file = open(txtpath,newline='')
    output_file = open(csvpath,'w')

    reader = csv.reader(input_file, delimiter='\t')
    writer = csv.writer(output_file, delimiter=',')

    for row in reader:
        writer.writerow(row)

    input_file.close()
    output_file.close()

def generate_postcodes(lookup: str, csvpath: str, output: str):
    """Build rust structs for each postcode record"""
    folder = Path(output)
    country = {}
    indices = {}
    export = ""
    codes = {}

    with open(lookup) as f:
        for line in f:
            line = line.strip().split(',')
            code = line[1]
            country[line[0]] = code
            indices[code] = 0

            export += f"pub mod {code.lower()};\n"

            codes[code] = []

    with open(folder / "mod.rs",'w') as f:
        f.write(export)

    with open(csvpath,newline='') as f:
        reader = csv.reader(f, delimiter=',')
        for row in reader:
            country_code = country[row[0]]
            index = indices[country_code]

            postal_code = row[1]
            place_name = row[2]
            admin_name1 = row[3]
            admin_code1 = row[4]
            admin_name2 = row[5]
            admin_code2 = row[6]
            admin_name3 = row[7]
            admin_code3 = row[8]

            latitude = 'None'
            longitude = 'None'
            accuracy = 'None'

            if len(row) > 9 and row[9].strip():
                value = float(row[9].strip())
                display = max('{:.2f}'.format(value),str(value),key=len)
                latitude = f"Some({display})"

            if len(row) > 10 and row[10].strip():
                value = float(row[10].strip())
                display = max('{:.2f}'.format(value),str(value),key=len)
                longitude = f"Some({display})"

            if len(row) > 11 and row[11].strip():
                accuracy = f"Some({row[11].strip()})"

            codes[country_code].append("( CountryCode::{}, \"{}\", \"{}\", \"{}\", \"{}\", \"{}\", \"{}\", \"{}\", \"{}\", {}, {}, {} )".format(
                country_code,
                postal_code,
                place_name,
                admin_name1,
                admin_code1,
                admin_name2,
                admin_code2,
                admin_name3,
                admin_code3,
                latitude,
                longitude,
                accuracy,
            ))

            indices[country_code] += 1

    for (code, records) in codes.items():
        path = folder / f"{code.lower()}.rs"
        path.unlink(missing_ok=True)
        count = len(records)

        with open(path,'w') as f:
            if count > 0:
                f.write("use isocountry::CountryCode;\n")

            f.write("use crate::CodeReference;\n")

            f.write(f"const CODES: [CodeReference;{count}] = [\n")

            for record in records:
                f.write(f"\t{record},\n")

            f.write("];")


if __name__=='__main__':
    download_postcodes(POSTCODES_URL, POSTCODES_TEXT)
    process_postcodes(POSTCODES_TEXT, POSTCODES_PATH)
    download_countries(COUNTRIES_URL)

    generate_postcodes(INDEX1_PATH,POSTCODES_PATH,POSTCODES_CODE)