[API Keys](/apikeys)

List API keys

# List API keys

This endpoint returns the API keys in [paginated](/#pagination) format.

## Request & Response


    GET https://proxy.webshare.io/api/v2/apikey/

PythonJavascriptcURL

example.py

    import requests

    response = requests.get(
        "https://proxy.webshare.io/api/v2/apikey/",
        headers={"Authorization": "Token APIKEY"}
    )
    response.json()

The commands above return JSON structured like this:

response.json

    {
      "count": 1,
      "next": null,
      "previous": null,
      "results": [
        {
          "id": 1337,
          "key": "abc1234...zzz",
          "label": "server1 key",
          "created_at": "2022-06-14T11:58:10.246406-07:00",
          "updated_at": "2022-06-14T11:58:10.246406-07:00"
        },
        ...
      ]
    }

[Retrieve API key](/apikeys/retrieve "Retrieve API key")[Delete API key](/apikeys/delete "Delete API key")
