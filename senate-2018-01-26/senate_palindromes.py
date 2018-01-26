"""
Code to scrape https://www.senate.gov/legislative/votes.htm
and get palindromic votes.
"""


import re
import requests
from bs4 import BeautifulSoup


def get_session_links():
    """
    Get the urls for each session of the Senate.
    """
    votes_url = 'https://www.senate.gov/legislative/votes.htm'
    response = requests.get(votes_url)
    soup_html = BeautifulSoup(response.text, 'html.parser')
    links = soup_html.select('.newspaperDisplay_3column > a')

    return {link.text: 'https://www.senate.gov' + link.get('href')
            for link in links}


def get_votes(session_link):
    """
    Get a list of the votes given a link to a session.
    """
    response = requests.get(session_link)
    soup_html = BeautifulSoup(response.text, 'html.parser')

    table = soup_html.find('table')
    votes = table.findAll('tr')[1:]
    vote_results = (vote.find('td').text for vote in votes)

    regex = re.compile(r'(\d+).\((\d+)-(\d+)\)')
    vote_results_regex = (regex.fullmatch(result) for result in vote_results)

    return {match.group(1): (match.group(2), match.group(3))
            for match in vote_results_regex}


def get_all_votes():
    """
    A dictionary of all the vote results.
    """
    votes_dict = {}
    for session, link in get_session_links().items():
        for vote_number, result in get_votes(link).items():
            votes_dict[session, vote_number] = result

    return votes_dict


def find_palindromes(votes):
    """
    Find palindromes in a dictionary of votes.
    """
    for vote, result in votes.items():
        string = result[0] + result[1]
        if string == string[::-1]:
            yield (vote, result)


def main():
    """
    Do everything
    """
    votes = get_all_votes()
    print(*find_palindromes(votes), sep='\n')
