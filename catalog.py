import requests
from bs4 import BeautifulSoup
import sys
import os
from dotenv import load_dotenv

# Check if the captcha code was provided as an argument
if len(sys.argv) < 2:
    print('Usage: catalog <captcha_code>')
    sys.exit(1)

# Load .env variables
load_dotenv()
username = os.getenv('USERNAME').upper()
password = os.getenv('PASSWORD')

# URL of the login page
login_url = "https://catalog.inf.elte.hu/Account/Login"

# Create a session to persist cookies
session = requests.Session()

# Get the login page to retrieve __AntiXsrfToken cookie
response = session.get(login_url)
soup = BeautifulSoup(response.text, 'html.parser')

# Extract hidden form fields
hidden_fields = {}
for input_tag in soup.find_all('input', type='hidden'):
    hidden_fields[input_tag['name']] = input_tag['value']

# Form data for the login request
login_data = {
    'ctl00$MainContent$Email':    username,
    'ctl00$MainContent$Password': password,
    'ctl00$MainContent$captcha':  sys.argv[1],
    "ctl00$MainContent$ctl06":	  "Log+in",
    "__EVENTTARGET":              "",
    "__EVENTARGUMENT":            "",
    **hidden_fields
}

# Send the POST request to login
response = session.post(login_url, data=login_data)

# Check if login was successful
if response.url == 'https://catalog.inf.elte.hu/Student/student':
    print('\033[92mLogin successful to Catalog!\033[0m')
else:
    print('\033[91mLogin failed to Catalog!\033[0m')