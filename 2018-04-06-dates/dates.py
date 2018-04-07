"""
Compute ranges of dates and incidences of vandalism.
"""


from datetime import datetime, timedelta


def date_range(start, end):
    """
    An generator of the dates in the interval [start, end).
    """
    current = start
    while current < end:
        yield current
        current += timedelta(1, 0, 0)


def vandalism(start, end):
    """
    An generator of the incidences of vandalism in the interval [start, end).
    """
    for date in date_range(start, end):
        if (date.year % 100) == date.day * date.month:
            yield date


def year_count(dates):
    """
    Return a dictionary of the years in the iterator
    along with the number of occurences of that year.
    """
    years = dict()
    for date in dates:
        years[date.year] = years.get(date.year, 0) + 1

    min_year = min(date.year for date in dates)
    max_year = max(date.year for date in dates)

    for year in range(min_year, max_year + 1):
        if year not in years:
            years[year] = 0

    return years


def main():
    """
    Get all the relevant info.
    """
    vandals = list(vandalism(datetime(2001, 1, 1), datetime(2100, 1, 1)))

    print("Number of incidents: {}".format(len(vandals)))

    years = year_count(vandals).items()

    min_year = min(years, key=lambda year: year[1])
    min_years = sorted(year[0] for year in years if year[1] == min_year[1])
    max_year = max(years, key=lambda year: year[1])
    max_years = sorted(year[0] for year in years if year[1] == max_year[1])

    print("Least incidents is {} in {}".format(min_year[1], min_years))
    print("Most incidents is {} in {}".format(max_year[1], max_years))

    gaps = list(zip(vandals, vandals[1:]))

    min_gap = min(gaps, key=lambda dates: dates[1] - dates[0])
    max_gap = max(gaps, key=lambda dates: dates[1] - dates[0])

    print("The shortest gap is between {} and {} at {} days".format(
        min_gap[0].isoformat()[:10],
        min_gap[1].isoformat()[:10],
        (min_gap[1] - min_gap[0]).days
    ))
    print("The longest gap is between {} and {} at {} days".format(
        max_gap[0].isoformat()[:10],
        max_gap[1].isoformat()[:10],
        (max_gap[1] - max_gap[0]).days
    ))


if __name__ == '__main__':
    main()
