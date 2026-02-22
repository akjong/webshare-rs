[Registration and Login](/registerandlogin)

Logout

# Logout

You can logout to invalidate the token you are using.

    POST https://proxy.webshare.io/api/v2/logout/

## Request & Response

PythonJavascriptcURL

logout.py

    import requests

    response = requests.get(
    "https://proxy.webshare.io/api/v2/logout/",
    headers={ "Authorization": "Token APIKEY" }
    )

    response.json()

the above command returns empty response with `204 No Content`

    HTTP/1.1 204 No Content

[Delete social account](/registerandlogin/delete-social-account "Delete social account")[Two-Factor Auth](/twofactorauth "Two-Factor Auth")
