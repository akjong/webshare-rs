[Proxy Configuration](/proxy-config)

Get proxy status

# Get proxy status

This endpoint retrieves the proxy status. You need to add a query-string parameter `plan_id`.

    GET https://proxy.webshare.io/api/v3/proxy/list/status?plan_id=<PLAN-ID>

## Request & Response

PythonJavascriptcURL

get_proxy_status.py

    import requests

    response = requests.get(
      "https://proxy.webshare.io/api/v3/proxy/list/status?plan_id=PLAN-ID",
      headers={ "Authorization": "Token APIKEY" }
    )

    response.json()

The commands above return JSON structured like this:

response.json

    {
      "state": "completed",
      "countries": {"US": 5, "FR": 100},
      "unallocated_countries": {},
      "username": "username",
      "password": "password",
      "is_proxy_used": false,
    }

[Get proxy stats](/proxy-config/get_proxy_stats "Get proxy stats")[Get proxy config](/proxy-config/get_proxy_config "Get proxy config")
