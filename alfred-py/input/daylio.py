import sys
import os
import argparse
import csv

import dotenv

dotenv.load_dotenv()


def printerr(*args, **kargs):
    print(*args, file=sys.stderr, **kargs)


argp = argparse.ArgumentParser(
    description="Process data from the app Daylio.",
)

argp.add_argument(
    "-f",
    "--daylio-csv-file",
    type=argparse.FileType("r"),
    default=os.getenv("DAYLIO_CSV_FILE"),
    help="""Path to the Daylio CSV file.
            Can be passed as environment variable: DAYLIO_CSV_FILE.""",
)


def main():
    args = argp.parse_args()
    print(args.daylio_csv_file)
    if args.daylio_csv_file is None:
        printerr("error: missing required argument DAYLIO_CSV_FILE.")
        sys.exit(1)

    csvfile = args.daylio_csv_file
    reader = csv.reader(csvfile)
    for row in reader:
        print(row)


if __name__ == "__main__":
    main()
